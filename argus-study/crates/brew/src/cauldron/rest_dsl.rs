use super::{
  Cauldron, Cold, Hot, MixingCauldron,
  Temperature, Warm,
};

pub trait RestDsl {
  type Output;

  fn rest(self) -> Self::Output;
}

impl<C> RestDsl for C
where
  C: Cauldron,
  C::Temperature: RestDsl,
  <C::Temperature as RestDsl>::Output:
    Temperature,
{
  type Output = MixingCauldron<
    C::IngredientCount,
    <C::Temperature as RestDsl>::Output,
  >;

  fn rest(self) -> Self::Output {
    MixingCauldron::new()
  }
}

impl RestDsl for Hot {
  type Output = Warm;

  fn rest(self) -> Self::Output {
    Warm(())
  }
}

impl RestDsl for Warm {
  type Output = Cold;

  fn rest(self) -> Self::Output {
    Cold(())
  }
}

impl RestDsl for Cold {
  type Output = Cold;

  fn rest(self) -> Self::Output {
    Cold(())
  }
}
