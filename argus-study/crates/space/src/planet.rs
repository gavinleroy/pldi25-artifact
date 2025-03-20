use crate::{num::*, pos::Pos};

pub trait Planet {
  type At: Pos;
}

pub struct Yavin;
pub struct Arrakis;
pub struct Darkover;
pub struct Coruscant;
pub struct Hoth;

impl Planet for Yavin {
  type At = (Two, Two);
}

impl Planet for Arrakis {
  type At = (Four, Three);
}

impl Planet for Darkover {
  type At = (Two, Five);
}

impl Planet for Coruscant {
  type At = (Five, Four);
}

impl Planet for Hoth {
  type At = (Seven, Four);
}
