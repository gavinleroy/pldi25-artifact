use std::{
  fmt::Debug, marker::PhantomData, rc::Rc,
};

use bevy_utils::all_tuples;

use crate::probe::ProbeParam;

pub struct IsRock;
pub struct IsDebris;

pub trait Rock {}
pub trait Debris {}

#[derive(Debug)]
pub struct Meteoroid(());

#[derive(Debug)]
pub struct Asteroid(());

#[derive(Debug)]
pub struct Meteorite(());

#[derive(Debug)]
pub struct AlienCrap(());

#[derive(Debug)]
pub struct Screw(());

#[derive(Debug)]
pub struct Bolt(());

#[derive(Debug)]
pub struct UFO(Rc<AlienCrap>);

pub trait Collectible {}

pub struct Col<T>(PhantomData<T>);

impl<T: Debug> Debug for Col<T> {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{:?}", self.0)
  }
}

impl<R: Collectible> ProbeParam for Col<R> {
  type Item = R;
}

impl Rock for Meteoroid {}
impl Rock for Meteorite {}
impl Rock for Asteroid {}
impl Debris for AlienCrap {}
impl Debris for Screw {}
impl Debris for Bolt {}
impl Debris for UFO {}

pub trait FinderData {
  type Item;
}

macro_rules! impl_query_data {
    ($($name: ident),*) => {
        $(impl FinderData for $name {
            type Item = $name;
        })*
    };
}

impl<D: FinderData> FinderData for Option<D> {
  type Item = Option<D::Item>;
}

impl_query_data! {
    Meteoroid,
    Meteorite,
    Asteroid,
    AlienCrap,
    Screw,
    Bolt,
    UFO
}

pub struct Finder<D: FinderData>(PhantomData<D>);

impl<D: FinderData> ProbeParam for Finder<D> {
  type Item = <D as FinderData>::Item;
}

macro_rules! impl_tuple_query_data {
    ($($name: ident),*) => {
        impl<$($name: FinderData),*> FinderData for ($($name,)*) {
            type Item = ($($name::Item,)*);
        }
    };
}

macro_rules! impl_tuple_rock {
    ($($name: ident),*) => {
        impl<$($name: Rock),*> Rock for ($($name,)*) {}
    };
}

macro_rules! impl_tuple_debris {
    ($($name: ident),*) => {
        impl<$($name: Debris),*> Debris for ($($name,)*) {}
    };
}

all_tuples!(impl_tuple_query_data, 1, 15, F);
all_tuples!(impl_tuple_rock, 1, 15, F);
all_tuples!(impl_tuple_debris, 1, 15, F);

pub struct QueryIter<D>(PhantomData<D>);

impl<D: FinderData> IntoIterator for &Finder<D> {
  type Item = <D as FinderData>::Item;
  type IntoIter =
    QueryIter<<D as FinderData>::Item>;

  fn into_iter(self) -> Self::IntoIter {
    QueryIter(PhantomData)
  }
}

impl<D> Iterator for QueryIter<D> {
  type Item = D;

  fn next(&mut self) -> Option<Self::Item> {
    None
  }
}
