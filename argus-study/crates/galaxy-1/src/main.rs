use space::prelude::*;

// NOTE: This logic is sufficient to collect the meteoroid,
// please don't change the function signature or body.
fn collect_meteoroid(query: Finder<Meteoroid>) {
  for m in &query {
    println!("Meteoroid collected! {m:?}");
  }
}

// TASK: a meteoroid has been reported north-east of the station. We need to travel
// some distance up and right to gather it, but I can't seem to get it right.
// Help me figure out how far to go!
fn main() {
  Rocket::from_origin()
    .up()
    .forward(One)
    .right()
    .forward(One)
    .probe(collect_meteoroid)
}
