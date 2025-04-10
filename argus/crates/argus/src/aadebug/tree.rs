use std::{cell::RefCell, ops::Deref, time::Instant};

use argus_ext::ty::{EvaluationResultExt, TyCtxtExt, TyExt};
use index_vec::IndexVec;
use rustc_infer::infer::InferCtxt;
use rustc_middle::{
  traits::solve::{CandidateSource, Goal as RGoal},
  ty::{self, TyCtxt},
};
use rustc_trait_selection::solve::inspect::ProbeKind;
use rustc_utils::timer;
use serde::Serialize;
#[cfg(feature = "testing")]
use ts_rs::TS;

use super::dnf::{And, Dnf};
use crate::{
  analysis::EvaluationResult,
  proof_tree::{topology::TreeTopology, ProofNodeIdx},
};

pub type I = ProofNodeIdx;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "testing", derive(TS))]
#[cfg_attr(feature = "testing", ts(export))]
pub struct SetHeuristic {
  pub momentum: usize,
  pub velocity: usize,
  goals: Vec<Heuristic>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "testing", derive(TS))]
#[cfg_attr(feature = "testing", ts(export))]
pub struct Heuristic {
  idx: I,
  kind: GoalKind,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[cfg_attr(feature = "testing", derive(TS))]
#[cfg_attr(feature = "testing", ts(export))]
enum Location {
  Local,
  External,
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
#[cfg_attr(feature = "testing", derive(TS))]
#[cfg_attr(feature = "testing", ts(export))]
enum GoalKind {
  Trait { _self: Location, _trait: Location },
  TyChange,
  FnToTrait { _trait: Location, arity: usize },
  TyAsCallable { arity: usize },
  DeleteFnParams { delta: usize },
  AddFnParams { delta: usize },
  // Represents a function with the correct number of parameters,
  // but the parameters trait bounds or types are unsatisifed.
  // TODO if it's worth the extra effort, we could figure out which
  // parameters are incorrect and highlight them to the user.
  IncorrectParams { arity: usize },
  Misc,
}

impl GoalKind {
  fn weight(&self) -> usize {
    use GoalKind as GK;
    use Location::{External as E, Local as L};
    match self {
      GK::Trait {
        _self: L,
        _trait: L,
      } => 0,

      GK::Trait {
        _self: L,
        _trait: E,
      }
      | GK::Trait {
        _self: E,
        _trait: L,
      }
      | GK::FnToTrait { _trait: L, .. } => 1,

      GK::Trait {
        _self: E,
        _trait: E,
      } => 2,

      GK::TyChange => 4,
      GK::IncorrectParams { arity: delta }
      | GK::AddFnParams { delta }
      | GK::DeleteFnParams { delta } => 5 * delta,
      GK::FnToTrait { _trait: E, arity }
      // You could implement the unstable Fn traits for a type,
      // we could thens suggest this if there's nothing else better.
      | GK::TyAsCallable { arity } => 4 + 5 * arity,
      GK::Misc => 50,
    }
  }
}

#[allow(clippy::struct_field_names)]
pub struct Goal<'a, 'tcx> {
  idx: I,
  result: EvaluationResult,
  tree: &'a T<'a, 'tcx>,
  infcx: &'a InferCtxt<'tcx>,
  goal: &'a RGoal<'tcx, ty::Predicate<'tcx>>,
}

impl From<Goal<'_, '_>> for I {
  fn from(val: Goal) -> Self {
    val.idx
  }
}

impl From<&Goal<'_, '_>> for I {
  fn from(val: &Goal) -> Self {
    val.idx
  }
}

impl<'a, 'tcx> Goal<'a, 'tcx> {
  fn all_candidates(&self) -> impl Iterator<Item = Candidate<'a, 'tcx>> + '_ {
    self
      .tree
      .topology
      .children(self.idx)
      .filter_map(move |i| self.tree.candidate(i))
  }

  fn interesting_candidates(
    &self,
  ) -> impl Iterator<Item = Candidate<'a, 'tcx>> + '_ {
    self.all_candidates().filter(|c| c.retain)
  }

  pub fn predicate(&self) -> ty::Predicate<'tcx> {
    self.goal.predicate
  }

  pub fn last_ancestor_pre_builtin(&self) -> Self {
    let mut i = self.idx;
    let tree = self.tree;

    let not_builtin = |kind| {
      !matches!(kind, ProbeKind::TraitCandidate {
        source: CandidateSource::BuiltinImpl(..),
        ..
      })
    };

    let get_next_ancestor = |i: I| -> Option<I> {
      let parent = tree.topology.parent(i)?;
      match tree.ns[parent] {
        N::C { kind, .. } if not_builtin(kind) => tree.topology.parent(parent),
        _ => None,
      }
    };

    while let Some(grandparent) = get_next_ancestor(i) {
      i = grandparent;
    }

    tree.goal(i).expect("invalid ancestor")
  }

  fn analyze(&self) -> Heuristic {
    use std::cmp::Ordering;

    // We should only be analyzing failed predicates
    assert!(!self.result.is_yes());

    log::debug!("ANALYZING {:?}", self.predicate());

    let tcx = self.infcx.tcx;

    let kind = match self.predicate().kind().skip_binder() {
      ty::PredicateKind::Clause(ty::ClauseKind::Trait(t))
        if t.polarity == ty::PredicatePolarity::Positive
          && tcx.is_fn_trait(t.def_id())
          && tcx.function_arity(t.self_ty()).is_some() =>
      {
        let fn_arity = tcx.function_arity(t.self_ty()).unwrap();
        let trait_arity = tcx.fn_trait_arity(t).unwrap_or(usize::MAX);

        log::debug!("FnSigs\n{:?}\n{:?}", t.self_ty(), t.trait_ref);
        log::debug!("Fn Args {:?}", t.trait_ref.args.into_type_list(tcx));
        log::debug!("{fn_arity} v {trait_arity}");

        match fn_arity.cmp(&trait_arity) {
          Ordering::Less => GoalKind::AddFnParams {
            delta: trait_arity - fn_arity,
          },
          Ordering::Greater => GoalKind::DeleteFnParams {
            delta: fn_arity - trait_arity,
          },
          Ordering::Equal => GoalKind::IncorrectParams { arity: fn_arity },
        }
      }

      // Self type is not callable but triat is in Fn family.
      ty::PredicateKind::Clause(ty::ClauseKind::Trait(t))
        if t.polarity == ty::PredicatePolarity::Positive
          && tcx.is_fn_trait(t.def_id()) =>
      {
        let trait_arity = tcx.fn_trait_arity(t).unwrap_or(usize::MAX);
        GoalKind::TyAsCallable { arity: trait_arity }
      }

      // Self type is a function type but the trait isn't
      ty::PredicateKind::Clause(ty::ClauseKind::Trait(t))
        if t.polarity == ty::PredicatePolarity::Positive
          && tcx.function_arity(t.self_ty()).is_some() =>
      {
        let fn_arity = tcx.function_arity(t.self_ty()).unwrap();
        let def_id = t.def_id();
        let location = if def_id.is_local() {
          Location::Local
        } else {
          Location::External
        };
        GoalKind::FnToTrait {
          _trait: location,
          arity: fn_arity,
        }
      }

      ty::PredicateKind::Clause(ty::ClauseKind::Trait(t))
        if t.polarity == ty::PredicatePolarity::Positive =>
      {
        log::debug!("Trait Self Ty {:?}", t.self_ty());

        let ty = t.self_ty();
        let def_id = t.def_id();

        let def_id_local = def_id.is_local();
        let ty_local = ty.is_local();

        match (ty_local, def_id_local) {
          (true, true) => GoalKind::Trait {
            _self: Location::Local,
            _trait: Location::Local,
          },
          (true, false) => GoalKind::Trait {
            _self: Location::Local,
            _trait: Location::External,
          },
          (false, true) => GoalKind::Trait {
            _self: Location::External,
            _trait: Location::Local,
          },
          (false, false) => GoalKind::Trait {
            _self: Location::External,
            _trait: Location::External,
          },
        }
      }

      ty::PredicateKind::Clause(ty::ClauseKind::Trait(t)) => {
        log::warn!("Trait Self Ty {:?} didn't match", t.self_ty());
        GoalKind::Misc
      }

      ty::PredicateKind::Clause(ty::ClauseKind::Projection(_)) => {
        GoalKind::TyChange
      }

      ty::PredicateKind::Clause(..)
      | ty::PredicateKind::NormalizesTo(..)
      | ty::PredicateKind::AliasRelate(..)
      | ty::PredicateKind::DynCompatible(..)
      | ty::PredicateKind::Subtype(..)
      | ty::PredicateKind::Coerce(..)
      | ty::PredicateKind::ConstEquate(..)
      | ty::PredicateKind::Ambiguous => GoalKind::Misc,
    };

    Heuristic {
      idx: self.idx,
      kind,
    }
  }
}

#[allow(dead_code)]
pub struct Candidate<'a, 'tcx> {
  idx: I,
  retain: bool,
  result: EvaluationResult,
  tree: &'a T<'a, 'tcx>,
  kind: &'a ProbeKind<TyCtxt<'tcx>>,
}

impl<'a, 'tcx> Candidate<'a, 'tcx> {
  fn all_subgoals(&self) -> impl Iterator<Item = Goal<'a, 'tcx>> + '_ {
    self
      .tree
      .topology
      .children(self.idx)
      .filter_map(move |i| self.tree.goal(i))
  }

  fn source_subgoals(&self) -> impl Iterator<Item = Goal<'a, 'tcx>> + '_ {
    let mut all_goals = self.all_subgoals().collect::<Vec<_>>();
    let cap = argus_ext::ty::retain_error_sources(
      &mut all_goals,
      |g| g.result,
      |g| g.goal.predicate,
      |g| g.infcx.tcx,
    );
    all_goals.truncate(cap);
    all_goals.into_iter()
  }
}

pub enum N<'tcx> {
  C {
    kind: ProbeKind<TyCtxt<'tcx>>,
    result: EvaluationResult,
    retain: bool,
  },
  R {
    infcx: InferCtxt<'tcx>,
    goal: RGoal<'tcx, ty::Predicate<'tcx>>,
    result: EvaluationResult,
  },
}

pub struct T<'a, 'tcx: 'a> {
  pub root: I,
  pub ns: &'a IndexVec<I, N<'tcx>>,
  pub topology: &'a TreeTopology,
  pub maybe_ambiguous: bool,
  report_performance: bool,
  dnf: RefCell<Option<Dnf<I>>>,
}

impl<'a, 'tcx: 'a> T<'a, 'tcx> {
  pub fn new(
    root: I,
    ns: &'a IndexVec<I, N<'tcx>>,
    topology: &'a TreeTopology,
    maybe_ambiguous: bool,
    report_performance: bool,
  ) -> Self {
    Self {
      root,
      ns,
      topology,
      maybe_ambiguous,
      report_performance,
      dnf: RefCell::new(None),
    }
  }

  pub fn for_correction_set(&self, mut f: impl FnMut(&And<I>)) {
    for and in self.dnf().iter_conjuncts() {
      f(and);
    }
  }

  pub fn goal(&self, i: I) -> Option<Goal<'_, 'tcx>> {
    match &self.ns[i] {
      N::R {
        infcx,
        goal,
        result,
      } => Some(Goal {
        idx: i,
        result: *result,
        tree: self,
        infcx,
        goal,
      }),
      N::C { .. } => None,
    }
  }

  pub fn candidate(&self, i: I) -> Option<Candidate<'_, 'tcx>> {
    match &self.ns[i] {
      N::C {
        kind,
        result,
        retain,
      } => Some(Candidate {
        idx: i,
        retain: *retain,
        result: *result,
        tree: self,
        kind,
      }),
      N::R { .. } => None,
    }
  }

  fn expect_dnf(&self) -> impl Deref<Target = Dnf<I>> + '_ {
    use std::cell::Ref;
    Ref::map(self.dnf.borrow(), |o| o.as_ref().expect("dnf"))
  }

  pub fn dnf(&self) -> impl Deref<Target = Dnf<I>> + '_ {
    fn goal_(this: &T, goal: &Goal) -> Option<Dnf<I>> {
      if !((this.maybe_ambiguous && goal.result.is_maybe())
        || goal.result.is_no())
      {
        return None;
      }

      let candidates = goal.interesting_candidates();
      let nested = candidates
        .filter_map(|c| candidate_(this, &c))
        .collect::<Vec<_>>();

      if nested.is_empty() {
        return Dnf::single(goal.idx).into();
      }

      Dnf::or(nested.into_iter())
    }

    fn candidate_(this: &T, candidate: &Candidate) -> Option<Dnf<I>> {
      if candidate.result.is_yes() {
        return None;
      }

      let goals = candidate.source_subgoals();
      Dnf::and(goals.filter_map(|g| goal_(this, &g)))
    }

    if self.dnf.borrow().is_some() {
      return self.expect_dnf();
    }

    let dnf_report_msg =
      format!("Normalizing to DNF from {} nodes", self.ns.len());
    let dnf_start = Instant::now();

    let root = self.goal(self.root).expect("invalid root");
    let dnf = goal_(self, &root).unwrap_or_else(Dnf::default);

    timer::elapsed(&dnf_report_msg, dnf_start);

    // HACK to gather the performance report we write to stderr the CSV values `PERF<NODES><TIME>`
    // The testing harness will take the stderr output and place it in a file for analysis.
    if self.report_performance {
      eprintln!(
        "PERF,{:?},{:.04}",
        self.ns.len(),
        dnf_start.elapsed().as_secs_f64()
      );
    }

    self.dnf.replace(Some(dnf));
    self.expect_dnf()
  }

  /// Failed predicates are weighted as follows.
  ///
  /// Each predicate is marked as local / external, local predicates are
  /// trusted less, while external predicates are assumed correct.
  ///
  /// Trait predicates `T: C`, are weighted by how much they could change.
  /// A type `T` that is local is non-rigid while external types are considered
  /// rigid, meaning they cannot be changed.
  ///
  /// Non-intrusive changes:
  ///
  /// A local type failing to implement a trait (local/external).
  /// NOTE that `T: C` where `T` is an external type is considered impossible
  /// to change, if this is the only option a relaxed rule might suggest
  /// creating a wrapper for the type.
  ///
  /// Intrusive changes
  ///
  /// Changing types. That could either be changing a type to match an
  /// alias-relate, deleting function parameters or tuple elements.
  pub fn weight(&self, and: &And<I>) -> SetHeuristic {
    let goals = and
      .iter()
      .map(|&idx| self.goal(idx).expect("goal").analyze())
      .collect::<Vec<_>>();

    let momentum = goals.iter().fold(0, |acc, g| acc + g.kind.weight());
    let velocity = and
      .iter()
      .map(|&idx| self.topology.depth(idx))
      .max()
      .unwrap_or(0);

    SetHeuristic {
      momentum,
      velocity,
      goals,
    }
  }
}

// ------------------
// Unimportant things

impl std::fmt::Debug for N<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      N::C {
        kind,
        result,
        retain,
      } => write!(f, "C {{ {retain} {result:?} {kind:?} }}"),
      N::R { goal, result, .. } => {
        write!(f, "R {{ result: {result:?}, goal: {:?} }}", goal.predicate)
      }
    }
  }
}
