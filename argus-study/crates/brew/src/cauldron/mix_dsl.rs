use std::ops::Add;

use super::{Cauldron, MixingCauldron};
use crate::{
  count::{Num, One},
  ingredient::Ingredient,
};

pub trait MixDsl<Rhs> {
  type Output;

  fn mix_with(self, rhs: Rhs) -> Self::Output;
}

impl<C, Rhs> MixDsl<Rhs> for C
where
  C: Cauldron,
  C::IngredientCount: Add<One>,
  <C::IngredientCount as Add<One>>::Output: Num,
  Rhs: Ingredient,
{
  type Output = MixingCauldron<
    <C::IngredientCount as Add<One>>::Output,
    C::Temperature,
  >;
  fn mix_with(self, _rhs: Rhs) -> Self::Output {
    MixingCauldron::new()
  }
}
