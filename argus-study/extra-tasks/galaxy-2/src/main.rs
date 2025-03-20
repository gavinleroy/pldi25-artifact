use space::prelude::*;

// TASK: I want to create a generic function that can take any probe
// configuration and perform an additional check for UFOs. The function
// body is sufficient and should not need modification, I just need help
// getting the type signature right.
fn probe_with_ufo_check<T, Marker>(
  r: T,
  config: impl IntoProbe<(), (), Marker>,
) where
  T: IntergalacticTravel,
{
  r.probe((
    config,
    |depot: &mut CollectionDeposit,
     ufo: Finder<Option<UFO>>| {
      for ufo in &ufo {
        if let Some(ufo) = ufo {
          println!("UFO sighting! {ufo:?}");
          depot.add_debris(ufo);
        }
      }
    },
  ))
}

// An example probe function that collects space debris.
fn collect_debris(query: Finder<(Bolt, Screw)>) {
  for (bolt, screw) in &query {
    println!(
      "Collecting: (bolt, screw) ({:?}, {:?})",
      bolt, screw
    );
  }
}

// Reports of a flying UFO have been reported in a cloud of debris at location (3, 0).
// Let's send a rocket to collect some of the debris and check for UFOs!
fn main() {
  let r =
    Rocket::from_origin().right().forward(Three);

  probe_with_ufo_check(r, collect_debris)
}
