mod botanicals;
mod cauldron;
mod count;
mod ingredient;
mod potions;
mod time;

pub mod prelude {
  pub use brew_macros::*;

  pub use crate::{
    botanicals::*, cauldron::*, ingredient::*,
    potions::*, time::*,
  };
}

#[macro_export]
macro_rules! unit_struct {
    ($($t:ident),*) => {
        $(
            pub struct $t(());
        )*
    }
}

#[macro_export]
macro_rules! impl_as {
    ($c:ident ==> $($t:tt),*) => {
        $(
            impl $c for $t {}
        )*
    }
}

#[macro_export]
macro_rules! make_simple {
    ($vis:vis $c:ident ==> $($t:tt),*) => {
        $crate::unit_struct!($($t),*);
        $vis trait $c {}
        $(
            impl $c for $t {}
        )*
    }
}

#[macro_export]
macro_rules! describe_recipe {
  ($fname:ident $ids:tt ==> $return_trait:ident) => {
    $crate::munch!($ids; []; $fname, $return_trait);
  }
}

#[macro_export]
macro_rules! munch {
  ([]; [$($ts:ty),*]; $f:ident, $ret:ident) => {
    fn $f<F, Res, Out>(func: F)
      where
        F: Fn($($ts),*) -> Res,
        Res: std::future::Future<Output = Out>,
        Out: $ret,
    {
      fn is_recipe<R>(_: impl IntoRecipe<R>) {}
      is_recipe(func);
    }
  };

  ([[botanical $b:ty] $($tts:tt)*]; [$($ts:ty),*]; $f:ident, $ret:ident) => {
    $crate::munch!([ $($tts)* ]; [$($ts,)* $b]; $f, $ret);
  };

  ([[mineral $b:ty] $($tts:tt)*]; [$($ts:ty),*]; $f:ident, $ret:ident) => {
    $crate::munch!([ $($tts)* ]; [$($ts,)* Fertilizer<$b>]; $f, $ret);
  };
}
