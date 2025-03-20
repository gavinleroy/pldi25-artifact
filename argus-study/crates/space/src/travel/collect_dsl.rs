use super::*;
use crate::{
  debris::{Debris, Rock},
  num::*,
  pos::PosEq,
  probe::IntoProbeConfigs,
};

pub trait CollectDsl<P, C, Marker> {
  fn collect(self, config: C);
}

pub struct IsDebrisCollector;
pub struct IsRockCollector;

impl<Item, R, P, C, L, X, Y>
  CollectDsl<P, C, IsDebrisCollector> for R
where
  X: IsEven + NonZero,
  Y: IsOdd + NonZero,
  Item: Debris,
  L: Pos<X = X, Y = Y>,
  R: IntergalacticTravel<Location = L>,
  C: IntoProbeConfigs<P, Item = Item>,
{
  fn collect(self, config: C) {
    todo!()
  }
}

impl<Item, R, P, C, L, X, Y>
  CollectDsl<P, C, IsRockCollector> for R
where
  X: IsOdd,
  Y: IsEven,
  Item: Rock,
  L: Pos<X = X, Y = Y>,
  R: IntergalacticTravel<Location = L>,
  C: IntoProbeConfigs<P, Item = Item>,
{
  fn collect(self, config: C) {
    todo!()
  }
}
