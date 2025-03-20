use std::marker::PhantomData;

use crate::botanicals::*;

pub(crate) trait PlantSafe:
  Ingredient
{
}
pub trait Ingredient {}

pub trait Mineral {}

pub struct Fertilizer<F>(PhantomData<F>);

impl<B> Ingredient for B {}

crate::impl_as! {
  PlantSafe ==>
  Dittany,
  Aconite,
  Wiggentree,
  Alihotsy,
  Shrivelfig,
  Bubotuber,
  Reflower
}

impl<M: Mineral> PlantSafe for Fertilizer<M> {}
