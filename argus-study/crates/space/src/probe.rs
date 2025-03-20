use std::{marker::PhantomData, sync::Arc};

use bevy_utils::all_tuples;

use crate::debris::{Debris, Rock};

pub struct SystemProbeTupleMarker;

pub trait IntoProbeConfigs<M>: Sized {
  type Item;
}

macro_rules! impl_probe_collection {
    ($(($param: ident, $sys: ident)),*) => {
        impl<$($param, $sys),*> IntoProbeConfigs<(SystemProbeTupleMarker, $($param,)*)> for ($($sys,)*)
        where
            $($sys: IntoProbeConfigs<$param>),*
        {
            type Item = ($($sys::Item,)*);
        }
    }
}

all_tuples!(impl_probe_collection, 1, 10, P, S);

pub trait IntoProbe<In, Out, Marker> {
  type Probe: Probe<In = In, Out = Out>;
}

impl<Marker, F> IntoProbeConfigs<Marker> for F
where
  F: IntoProbe<(), (), Marker>,
{
  type Item = <F::Probe as Probe>::Item;
}

pub trait Probe {
  type In;
  type Out;
  type Item;
}

impl<T>
  IntoProbe<
    <T as Probe>::In,
    <T as Probe>::Out,
    (),
  > for T
where
  T: Probe,
{
  type Probe = T;
}

pub struct IsProbeFunction;
pub struct IsExclusiveProbeFunction;

pub trait ProbeParam {
  type Item;
}

macro_rules! impl_system_param_tuple {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<$($param: ProbeParam),*> ProbeParam for ($($param,)*) {
            type Item = ($($param::Item,)*);
        }
    };
}

all_tuples!(impl_system_param_tuple, 0, 16, P);

pub struct FunctionProbe<M, F: ProbeFunction<M>>(
  PhantomData<Arc<(M, F)>>,
);

impl<Marker, F> Probe for FunctionProbe<Marker, F>
where
  F: ProbeFunction<Marker>,
{
  type In = F::In;
  type Out = F::Out;
  type Item = <F::Param as ProbeParam>::Item;
}

pub trait ProbeFunction<M>: Sized {
  type In;
  type Out;
  type Param: ProbeParam;
}

impl<Marker, F>
  IntoProbe<
    F::In,
    F::Out,
    (IsProbeFunction, Marker),
  > for F
where
  F: ProbeFunction<Marker>,
{
  type Probe = FunctionProbe<Marker, F>;
}

impl<Marker, F>
  IntoProbe<
    F::In,
    F::Out,
    (IsExclusiveProbeFunction, Marker),
  > for F
where
  F: ExclusiveProbeFunction<Marker>,
{
  type Probe = ExclusiveFunctionProbe<Marker, F>;
}

macro_rules! impl_probe_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func, $($param: ProbeParam),*> ProbeFunction<fn($($param,)*) -> Out> for Func
          where
            for <'a> &'a mut Func: FnMut($($param),*) -> Out + Send + Sync,
            ($($param,)*): Send + Sync,
            Out: 'static
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);
        }
    };
}

all_tuples!(impl_probe_function, 0, 16, F);

pub struct CollectionDeposit(());

impl CollectionDeposit {
  pub fn add_debris(
    &mut self,
    _debris: impl Debris,
  ) {
  }
  pub fn add_rock(&mut self, _rock: impl Rock) {}
}

pub trait ExclusiveProbeFunction<M>:
  Sized
{
  type In;
  type Out;
  type Param: ProbeParam;
}

pub struct ExclusiveFunctionProbe<
  M,
  F: ExclusiveProbeFunction<M>,
>(PhantomData<Arc<(M, F)>>);

impl<Marker, F> Probe
  for ExclusiveFunctionProbe<Marker, F>
where
  F: ExclusiveProbeFunction<Marker>,
{
  type In = F::In;
  type Out = F::Out;
  type Item = <F::Param as ProbeParam>::Item;
}

macro_rules! impl_exclusive_probe_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Out, Func, $($param: ProbeParam),*> ExclusiveProbeFunction<fn(&mut CollectionDeposit, $($param,)*) -> Out> for Func
          where
            for <'a> &'a mut Func: FnMut(&'a mut CollectionDeposit, $($param),*) -> Out,
            Out: 'static,
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);
        }
    };
}

all_tuples!(
  impl_exclusive_probe_function,
  0,
  16,
  F
);
