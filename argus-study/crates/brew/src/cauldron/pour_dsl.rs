use super::{Cauldron, Cold, NonEmpty, Warm};
use crate::{count::*, potions::*};

pub struct IsColdRemedy;
pub struct IsWarmRemedy;
pub struct IsColdPoison;
pub struct IsWarmPoison;

pub trait PourDsl<P: Potion, Marker> {
  /// The type returned by the `.pour()` method.
  type Output;

  fn pour(self) -> Self::Output;
}

impl<C, R> PourDsl<R, IsColdRemedy> for C
where
  R: Remedy,
  C::IngredientCount: NonEmpty + IsOdd,
  C: Cauldron<Temperature = Cold>,
{
  type Output = R;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, R> PourDsl<R, IsWarmRemedy> for C
where
  R: Remedy,
  C::IngredientCount: NonEmpty + IsEven,
  C: Cauldron<Temperature = Warm>,
{
  type Output = R;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, P> PourDsl<P, IsColdPoison> for C
where
  P: Poison,
  C::IngredientCount: NonEmpty + IsEven,
  C: Cauldron<Temperature = Cold>,
{
  type Output = P;

  fn pour(self) -> Self::Output {
    todo!()
  }
}

impl<C, P> PourDsl<P, IsWarmPoison> for C
where
  P: Poison,
  C::IngredientCount: NonEmpty + IsOdd,
  C: Cauldron<Temperature = Warm>,
{
  type Output = P;

  fn pour(self) -> Self::Output {
    todo!()
  }
}
