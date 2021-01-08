use std::ops::Add;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// `length` is a type with phantom type parameter `Unit`,
// and is not generic over the length type (that is `f64`)

// `f64` already implements the `Clone` and `Copy` traits.
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// The `Add` trait defines the behaviour of the `+` operator.
impl<Unit> Add for Length<Unit> {
  type Output = Length<Unit>;

  // add() returns a new `Length` struct containing the sum.
  fn add(self, rhs: Length<Unit>) -> Length<Unit> {
    Length(self.0 + rhs.0, PhantomData)
  }
}

fn main() {
  // Specifies `one_foot` to have phantom type parameter `Inch`.
  let one_foot: Length<Inch> = Length(12.0, PhantomData);
  // `one_meter` has phantom type parameter `Mm`.
  let one_meter: Length<Mm> = Length(1000.0, PhantomData);

  // `+` calls the `add()` method we implemented for `Length<Unit>`.

  let two_feet = one_foot + one_foot;
  let two_meters = one_meter + one_meter;

  // Addition works
  println!("one foot + one foot = {:?} in", two_feet.0);
  println!("one meter + one meter = {:?} mm", two_meters.0);
}