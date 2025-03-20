use std::marker::PhantomData;

use crate::{
  dir::{Direction, Down, Left, Right, Up},
  num::{self, Num},
  pos::{Origin, Pos},
};

mod blast_dsl;
mod collect_dsl;
mod fuel_dsl;
mod move_dsl;

mod types {
  use super::*;
  pub type Move<Rocket, D> =
    <Rocket as move_dsl::MoveDsl<D>>::Output;
  pub type Fuel<Rocket, N, P> =
    <Rocket as fuel_dsl::FuelDsl<N, P>>::Output;
  pub type Blast<Rocket, C> =
    <Rocket as blast_dsl::BlastDsl<C>>::Output;
}

mod methods {
  pub use super::{
    blast_dsl::*,
    collect_dsl::CollectDsl as ProbeDsl,
    fuel_dsl::*, move_dsl::*,
  };
}

pub(crate) type MaxFuel = num::Four;

pub trait IntergalacticTravel {
  type Location: Pos;
  type Fuel: Num;
  type Dir: Direction;
}

pub trait ExplorationDsl: Sized {
  fn left(self) -> types::Move<Self, Left>
  where
    Self: methods::MoveDsl<Left>,
  {
    methods::MoveDsl::r#move(self, Left)
  }

  fn right(self) -> types::Move<Self, Right>
  where
    Self: methods::MoveDsl<Right>,
  {
    methods::MoveDsl::r#move(self, Right)
  }

  fn up(self) -> types::Move<Self, Up>
  where
    Self: methods::MoveDsl<Up>,
  {
    methods::MoveDsl::r#move(self, Up)
  }

  fn down(self) -> types::Move<Self, Down>
  where
    Self: methods::MoveDsl<Down>,
  {
    methods::MoveDsl::r#move(self, Down)
  }

  fn forward<X>(
    self,
    x: X,
  ) -> types::Blast<Self, X>
  where
    Self: methods::BlastDsl<X>,
  {
    methods::BlastDsl::blast(self, x)
  }

  fn refuel<N, P>(
    self,
    p: P,
    x: N,
  ) -> types::Fuel<Self, N, P>
  where
    Self: methods::FuelDsl<N, P>,
  {
    methods::FuelDsl::refuel(self, p, x)
  }

  fn probe<P, C, Marker>(self, config: C)
  where
    Self: methods::ProbeDsl<P, C, Marker>,
  {
    methods::ProbeDsl::collect(self, config);
  }
}

impl<R: IntergalacticTravel> ExplorationDsl
  for R
{
}

pub struct Rocket<Loc, F, Dir>(
  PhantomData<(Loc, F, Dir)>,
);

impl Rocket<Origin, MaxFuel, Up> {
  pub fn from_origin() -> Self {
    Rocket(PhantomData)
  }
}

impl<L: Pos, F: Num, Dir: Direction>
  IntergalacticTravel for Rocket<L, F, Dir>
{
  type Location = L;
  type Fuel = F;
  type Dir = Dir;
}
