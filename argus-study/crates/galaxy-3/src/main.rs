use space::prelude::*;

// NOTE: The collection method should be sufficient and does not need modification.
fn collect_debris(
  query: Finder<(Bolt, AlienCrap)>,
) {
  for (b, a) in &query {
    println!(
      "Bolt & AlienCrap collected! {b:?} {a:?}"
    );
  }
}

// TASK: A satellite crashed into a UFO leaving a pile of debris
// and alien junk at location (2, 5) and we need to go clean it up.
// My flight plan isn't compiling for some reason...
fn main() {
  Rocket::from_origin()
    .up()
    .forward(Two)
    .right()
    .forward(One)
    .refuel(Yavin, Four)
    .up()
    .forward(Three)
    .right()
    .forward(One)
    .probe(collect_debris)
}
