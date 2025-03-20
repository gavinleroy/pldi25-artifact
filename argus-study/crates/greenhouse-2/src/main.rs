use brew::prelude::*;

// NOTE: my spicy recipe has been in the family for generations, no
// need to change something that isn't broken.
async fn spicy(
  i1: Shrivelfig,
  i2: Shrivelfig,
  i3: Dittany,
) -> Pink {
  EmptyCauldron::new()
    .mix(i1)
    .boil()
    .mix(i2)
    .mix(i3)
    .rest()
    .pour_as()
}

// NOTE: my neighbors insist on using their boring recipe. It helps their
// plants grow and I don't want them to suspect faul-play, so don't change it.
async fn boring(
  i1: Reflower,
  i2: Dittany,
) -> Blue {
  EmptyCauldron::new()
    .mix(i1)
    .mix(i2)
    .boil()
    .rest()
    .pour_as()
}

fn sabotage_feedings<T, const N: usize, R, S>(
  garden: &mut Garden<T, N>,
  recipe: R,
) where
  R: IntoRecipe<S>,
{
  garden.add_feeding_schedule(Daily, recipe);
  garden.add_feeding_schedule(Weekly, spicy);
}

// TASK: My neighbors have a garden of shrivelfigs that keeps growing,
// and growing, and GROWING! It's getting out of control. This weekend I plan
// to restrict their growth by slipping them a little concoction I've whipped up,
// my secret spicy potion recipe.  The recipe is simple, it returns a pink
// potion that takes 2 parts shrivelfig, and 1 part dittany. The secret: It must be boiled.
fn main() {
  let g = &mut Garden::<Shrivelfig, 12>::new();
  sabotage_feedings(g, boring);

  g.garden();
}
