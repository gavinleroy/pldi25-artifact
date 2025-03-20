//! Potion brewing system.

use std::future::Future;

use crate::ingredient::PlantSafe;

// --------------
// Potions

pub trait Potion {}
pub trait Poison: Potion {}
pub trait Remedy: Potion {}
pub trait IntoPotion {
  type Output: Potion;
}

impl<P: Potion> IntoPotion for P {
  type Output = P;
}

crate::unit_struct! {
    Green,
    Blue,
    Pink,
    Yellow
}

crate::impl_as! {
    Potion ==>
    Green,
    Blue,
    Pink,
    Yellow
}

crate::impl_as! {
    Poison ==>
    Pink,
    Yellow
}

crate::impl_as! {
    Remedy ==>
    Green,
    Blue
}

// --------------
// Recipes

pub trait IntoRecipe<T> {
  type Output: Potion;
}

impl<IP: IntoPotion> IntoRecipe<()> for IP {
  type Output = IP::Output;
}

impl<F, T1, Out, Res> IntoRecipe<(T1, Out, Res)>
  for F
where
  F: FnOnce(T1) -> Out,
  T1: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

impl<F, T1, T2, Out, Res>
  IntoRecipe<(T1, T2, Out, Res)> for F
where
  F: FnOnce(T1, T2) -> Out,
  T1: PlantSafe,
  T2: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

impl<F, T1, T2, T3, Out, Res>
  IntoRecipe<(T1, T2, T3, Out, Res)> for F
where
  F: FnOnce(T1, T2, T3) -> Out,
  T1: PlantSafe,
  T2: PlantSafe,
  T3: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

impl<F, T1, T2, T3, T4, Out, Res>
  IntoRecipe<(T1, T2, T3, T4, Out, Res)> for F
where
  F: FnOnce(T1, T2, T3, T4) -> Out,
  T1: PlantSafe,
  T2: PlantSafe,
  T3: PlantSafe,
  T4: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

impl<F, T1, T2, T3, T4, T5, Out, Res>
  IntoRecipe<(T1, T2, T3, T4, T5, Out, Res)> for F
where
  F: FnOnce(T1, T2, T3, T4, T5) -> Out,
  T1: PlantSafe,
  T2: PlantSafe,
  T3: PlantSafe,
  T4: PlantSafe,
  T5: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

impl<F, T1, T2, T3, T4, T5, T6, Out, Res>
  IntoRecipe<(T1, T2, T3, T4, T5, T6, Out, Res)>
  for F
where
  F: FnOnce(T1, T2, T3, T4, T5, T6) -> Out,
  T1: PlantSafe,
  T2: PlantSafe,
  T3: PlantSafe,
  T4: PlantSafe,
  T5: PlantSafe,
  T6: PlantSafe + Send,
  Out: Future<Output = Res>,
  Res: IntoPotion,
{
  type Output = Res::Output;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::botanicals::*;

  fn is_recipe<R: IntoRecipe<T>, T>(_: R) {}

  #[test]
  fn test_1() {
    async fn f(_: Dittany) -> Blue {
      todo!()
    }
    is_recipe(f);
  }

  #[test]
  fn test_2() {
    async fn f(
      _: Dittany,
      _: Aconite,
      _: Wiggentree,
    ) -> Blue {
      todo!()
    }
    is_recipe(f);
  }

  #[test]
  fn test_3() {
    async fn f(
      _: Dittany,
      _: Wiggentree,
    ) -> Pink {
      todo!()
    }
    is_recipe(f);
  }
}
