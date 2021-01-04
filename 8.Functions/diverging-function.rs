// Diverging function are never returned and are marked with `!` which is an empty type
fn foo() -> ! {
  panic!("This call never returns");
}

// `!` is different from `()` type, `()` has one possible value, but the number of possible
// values `!` has is none

fn some_fn() -> () {
  ()
}

fn main() {
  let x: ! = panic!("This call never returns.");
  println!("You will never see this line!");

  fn some_of_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
      let addition: u32 = match i%2 == 1 {
        true => i,
        false => continue,
      };
      acc += addition;
    }
    acc;
  }

  println!("Sum of odd numbers up to 9 (excluding): {}", some_of_odd_numbers(9));
}