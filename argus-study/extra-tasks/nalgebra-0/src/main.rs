use nalgebra::Vector2;
use nalgebra_sparse::{CooMatrix, CsrMatrix};

fn main() {
  let coo1 = CooMatrix::<Vector2<i32>>::new(5, 5);
  let m1 = CsrMatrix::from(&coo1);

  println!("{:?}", m1 * m1);
}
