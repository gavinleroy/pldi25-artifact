use brew::prelude::*;

#[derive(Mineral)]
struct Nitrogen;

async fn supplement(
  i1: Reflower,
  i2: Fertilizer<Nitrogen>,
) -> Blue {
  EmptyCauldron::new()
    .mix(i1)
    .boil()
    .mix(i2)
    .rest()
    .rest()
    .pour_as()
}

// TASK: Create a garden of alihotsy and feed them yearly with the following
// The recipe takes 1 parts reflower and 1 part nitrogen fertilizer, the result
// is a blue potion that really makes the alihotsies grow!
fn main() {
  Garden::<Alihotsy, 2>::new()
    .add_feeding_schedule(Yearly, supplement)
    .garden()
}
