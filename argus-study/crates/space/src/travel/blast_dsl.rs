use std::ops::{Add, Sub};

use super::*;
use crate::{
  dir::Direction, num::LessOrEqual,
  pos::PosUpdate,
};

pub trait BlastDsl<X> {
  type Output;

  fn blast(self, x: X) -> Self::Output;
}

impl<R, X> BlastDsl<X> for R
where
  R: IntergalacticTravel,
  X: Num,
  X: LessOrEqual<R::Fuel>,
  R::Location: PosUpdate<R::Dir, X>,
  R::Fuel: Sub<X>,
{
  type Output = Rocket<
    <R::Location as PosUpdate<R::Dir, X>>::Output,
    <R::Fuel as Sub<X>>::Output,
    R::Dir,
  >;

  fn blast(self, x: X) -> Self::Output {
    todo!()
  }
}
