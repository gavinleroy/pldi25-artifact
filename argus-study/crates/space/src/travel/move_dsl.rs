use super::*;
use crate::{dir::Direction, num::IsZero};

pub trait MoveDsl<D> {
  type Output;

  fn r#move(self, d: D) -> Self::Output;
}

impl<R, D> MoveDsl<D> for R
where
  R: IntergalacticTravel,
  D: Direction,
{
  type Output = Rocket<R::Location, R::Fuel, D>;

  fn r#move(self, d: D) -> Self::Output {
    todo!()
  }
}
