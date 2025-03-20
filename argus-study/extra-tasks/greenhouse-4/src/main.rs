use brew::prelude::*;
use rayon::prelude::*;

const FARM_SIZE: usize = 2048;
const PLANTER_SIZE: usize = 4;

async fn fertilize(
  i1: Dittany,
  i2: Shrivelfig,
  i3: Reflower,
) -> Blue {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .mix(i3)
    .pour_as()
}

// TASK: We have a *farm* of alihotsy, 2048 gardens with 4 plants in each. It takes far too
// long to fertilize them one by one, so we need to fertilize them in parallel. Our family
// recipe works best:
// - 1 part dittany
// - 1 part shrivelfig
// - 1 part reflower
// you mix the ingredients cold and get a beautiful blue potion that is perfect for the alihotsy.
fn main() {
  let mut gardens = vec![];
  gardens.resize_with(FARM_SIZE, || {
    Garden::<Alihotsy, PLANTER_SIZE>::new()
  });

  gardens.par_chunks(32).for_each(|gardens| {
    gardens.feed_in_parallel(fertilize)
  })
}
