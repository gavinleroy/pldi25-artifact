//! Development Task: finish TODO items and make the tests pass.
#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use brew::prelude::*;
use brew_macros::*;

fn main() {
  Garden::<Dittany, 1024>::new()
    // TODO: make a new recipe, called `secret_recipe` that combines the
    // following ingredients and pours into a `Pink` potion. This should be used
    // as a `Daily` feeding schedule.
    //
    // - Shrivelfig
    // - Wiggentree
    //
    // NOTE: the recipe also says that the ingredients must be boiled at least once.
    .garden()
}
