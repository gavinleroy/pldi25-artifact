use std::ops::Add;

use crate::{
  dir::{Direction, Right, Up},
  num::{IsEven, IsOdd, Num, NumEq, Zero},
};

pub trait Pos {
  type X: Num;
  type Y: Num;
}

pub type Origin = (Zero, Zero);

pub trait PosUpdate<D: Direction, X: Num> {
  type Output: Pos;
}

impl<N, P> PosUpdate<Up, N> for P
where
  N: Num,
  P: Pos,
  P::Y: Add<N>,
  <P::Y as Add<N>>::Output: Num,
{
  type Output = (P::X, <P::Y as Add<N>>::Output);
}

impl<N, P> PosUpdate<Right, N> for P
where
  N: Num,
  P: Pos,
  P::X: Add<N>,
  <P::X as Add<N>>::Output: Num,
{
  type Output = (<P::X as Add<N>>::Output, P::Y);
}

impl<X: Num, Y: Num> Pos for (X, Y) {
  type X = X;
  type Y = Y;
}

pub trait PosEq<P: Pos> {}

impl<P1: Pos, P2: Pos> PosEq<P2> for P1
where
  P1::X: NumEq<P2::X>,
  P1::Y: NumEq<P2::Y>,
{
}

pub trait OddEvenPos: Pos {}
impl<X, Y, L> OddEvenPos for L
where
  L: Pos<X = X, Y = Y>,
  X: IsOdd,
  Y: IsEven,
{
}

pub trait EvenOddPos: Pos {}
impl<X, Y, L> EvenOddPos for L
where
  L: Pos<X = X, Y = Y>,
  X: IsEven,
  Y: IsOdd,
{
}
