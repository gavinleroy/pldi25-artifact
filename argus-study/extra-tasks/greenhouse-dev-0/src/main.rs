//! Development Task: finish TODO items and make the tests pass.
#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use brew::prelude::*;
use brew_macros::*;

fn main() {
  Garden::<Alihotsy, 1>::new()
    // TODO: make a new recipe, called `secret_recipe` that combines
    // the following ingredients and pours as a `Remedy`.
    //
    // - Reflower
    // - Phosphorus
    // - Alihotsy
    // - Shrivelfig
    //
    // This recipe should be fed to the garden on a `Yearly` basis.
    .garden()
}
