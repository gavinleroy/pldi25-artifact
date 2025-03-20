use space::prelude::*;

// NOTE: The probe must collect screws, bolts, and the reported UFO.
// The type of `query` should not need modification.
fn collect_debris(
  query: Finder<(Screw, Bolt, UFO)>,
) {
  for (s, b, ufo) in &query {
    println!("Collecting debris: (screw, bolt, ufo) ({s:?}, {b:?}, {ufo:?})");
  }
}

// TASK: a crashed UFO was reported at location (2, 1). We need to
// send a probe to collect the debris for study. The following flight
// plan is correct and shouldn't be modified.
fn main() {
  Rocket::from_origin()
    .up()
    .forward(One)
    .right()
    .forward(Two)
    .probe(collect_debris)
}
