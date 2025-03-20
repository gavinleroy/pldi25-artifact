use super::{
  Cauldron, Hot, MixingCauldron, NonEmpty,
};

pub trait BoilDsl {
  type Output;

  fn boil(self) -> Self::Output;
}

impl<C> BoilDsl for C
where
  C: Cauldron,
  C::IngredientCount: NonEmpty,
{
  type Output =
    MixingCauldron<C::IngredientCount, Hot>;

  fn boil(self) -> Self::Output {
    MixingCauldron::new()
  }
}
