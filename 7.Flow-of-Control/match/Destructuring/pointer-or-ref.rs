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
}