/* Important note
  `*` - this is a dereference operator
  `&` - This operator can be used for destructuring
*/
fn main() {
  // Assigning a reference
  let reference = &4;

  match reference {
    /* If `reference` is a pattern matched against `&val`, it results in a comparison like:
      `&i32`
      `&val`
      We see if the matching & are dropped, then the i32 should be assigned to va
    */
    &val => println!("Got a value via destructuring: {:?}", val),
  }

  // To avoid the `&`, you dereference before matching.
  match *reference {
    val => println!("Got a value via dereferencing: {:?}", val),
  }

  // What if you didn't start with a refernce? `reference` was a `&
  // because the right side was already a reference. This is not
  // a reference because the right side is not one.
  let _not_a_reference = 3;

  // Rust provides `ref` for exactly this purpose. It modifies the
  // assignment so that a reference is created for a element; this
  // reference is assigned
  let ref _is_a_reference = 3;

  // Accordingly, by defining 2 values without a references, references
  // can be retrieved via `ref` and `ref mut`.
  let value = 5;
  let mut mut_value = 6;

  // Using keyword `ref` to create a reference
  match value {
    ref r => println!("Got a reference to a value: {:?}", r),
  }

  // Using `ref mut` similarly.
  match mut_value {
    ref mut m => {
      // Got a reference, so we dereference here
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m);
    },
  }
}