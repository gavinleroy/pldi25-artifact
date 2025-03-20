use std::ops::{Add, Sub};

use num::One;

use super::*;
use crate::{
  dir::Direction, num::LessOrEqual,
  planet::Planet,
};

pub trait FuelDsl<N, P> {
  type Output;

  fn refuel(self, p: P, n: N) -> Self::Output;
}

impl<R, N, P> FuelDsl<N, P> for R
where
  P: Planet,
  R: IntergalacticTravel<Location = P::At>,
  R::Fuel: LessOrEqual<One>,
  R::Fuel: Add<N>,
  <R::Fuel as Add<N>>::Output:
    LessOrEqual<MaxFuel>,
{
  type Output = Rocket<
    R::Location,
    <R::Fuel as Add<N>>::Output,
    R::Dir,
  >;

  fn refuel(self, p: P, n: N) -> Self::Output {
    todo!()
  }
}
