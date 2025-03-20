use entrait::*;

#[entrait(T1)]
fn t1(deps: &impl T2) {
  deps.t2();
}

#[entrait(T2)]
fn t2(deps: &impl T3) {
  deps.t3();
}

#[entrait(T3)]
fn t3(deps: &impl T4) {}

trait T4 {}

fn main() {
  let app = Impl::new(());
  app.t1();
}
