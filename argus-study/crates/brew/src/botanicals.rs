//! Botanicals grown for use in potions.
use crate::{
  potions::{IntoRecipe, Poison},
  time::AsSchedule,
};

crate::make_simple! {
  pub(crate) Botanical ==>
  Dittany,
  Aconite,
  Wiggentree,
  Alihotsy,
  Shrivelfig,
  Bubotuber
}

pub struct Reflower(
  std::marker::PhantomData<std::rc::Rc<()>>,
);
impl Botanical for Reflower {}

pub trait Feedable {}
impl<B: Botanical> Feedable for B {}

pub trait AsGarden {}
impl<T: Botanical, const N: usize> AsGarden
  for Garden<T, N>
{
}

pub struct Garden<T: Feedable, const N: usize> {
  _plants: std::marker::PhantomData<[T; N]>,
}

impl<T: Feedable, const N: usize> Garden<T, N> {
  pub fn new() -> Self {
    Garden {
      _plants: std::marker::PhantomData,
    }
  }

  pub fn add_feeding_schedule<R>(
    &mut self,
    _schedule: impl AsSchedule,
    _recipe: impl IntoRecipe<R>,
  ) -> &mut Self {
    self
  }

  pub fn sabotage<P, S, R>(
    &mut self,
    _recipe: R,
  ) -> &mut Self
  where
    P: Poison,
    R: IntoRecipe<S, Output = P>,
  {
    self
  }

  pub fn garden(&self) {}
}

pub trait ParallelFeed<M> {
  fn feed_in_parallel<S, R>(&self, _recipe: R)
  where
    R: IntoRecipe<S> + Send + Sync,
    R::Output: Send + Sync;
}

impl<'a, G: AsGarden, V: AsRef<[G]>>
  ParallelFeed<G> for V
{
  fn feed_in_parallel<S, R>(&self, _recipe: R)
  where
    R: IntoRecipe<S> + Send + Sync,
    R::Output: Send + Sync,
  {
    ()
  }
}
