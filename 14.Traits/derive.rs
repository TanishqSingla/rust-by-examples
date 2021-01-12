/* Following is the list of some traits via #[derive] attribute
1. Comparison traits: `Eq`, `Partial Eq`, `Ord`, `PartialOrd`.
2. `Clone`, to create `T` from `&T` via copy
3. `Copy`, to give a type 'copy semantics' instead of 'move semantics'
4. `Hash`, to compute a hash from `&T`
5. `Default`, to create an empty instance of data type.
6. `Debug`, to format a value using the `{:?}` formatter.
*/

// `Centimeter`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
  fn to_centimeters(&self) -> Centimeters {
    let &Inches(inches) = self;

    Centimeters(inches as f64 * 2.54)
  }
}

// `Seconds`, a tuple struct with no addtional attributes
struct Seconds(i32);

fn main() {
  let _one_second = Seconds(1);

  // Error: Seconds can't be printed it doesn't implement the `Debug` trait
  // println!("one second looks like: {:?}" _one_second);

  // Error: Seoconds can't be compared
  // let _this_is_true = (_one_second == _one_second);

  let foot = Inches(12);

  println!("One foot equals {:?}", foot);

  let meter = Centimeters(100.0);

  let cmp = 
      if foot.to_centimeters() < meter {
        "smalleer"
      } else {
        "bigger"
      };
  
  println!("One foot is {} than one meter.", cmp);
}