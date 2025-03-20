#![allow(
  unused_variables,
  dead_code,
  unused_imports
)]

mod debris;
mod dir;
mod num;
mod planet;
mod pos;
mod probe;
mod travel;

pub mod prelude {
  pub use space_macros::*;

  pub use crate::{
    debris::*, dir::*, num::*, planet::*, pos::*,
    probe::*, travel::*,
  };
}
