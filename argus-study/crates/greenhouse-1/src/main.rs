use brew::prelude::*;

// NOTE: I've been working on this recipe for a while now, but still can't get it to type check!
async fn magic_brownie(
  i1: Dittany,
  i2: Bubotuber,
) -> Green {
  EmptyCauldron::new()
    .mix(i2)
    .boil()
    .mix(i1)
    .pour_as()
}

// TASK: Create a garden of wiggentrees and feed them monthly with the following recipe.
// The resulting potion should be green, taking 1 parts dittany and 1 parts bubotuber,
// there was no specification on the required temperature.
// The below setup code should not need modification.
fn main() {
  Garden::<Wiggentree, 12>::new()
    .add_feeding_schedule(Monthly, magic_brownie)
    .garden()
}
