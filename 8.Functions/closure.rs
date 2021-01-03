fn main() {
  fn function (i: i32) -> i32 {i + 1}

  let closure_annotated = |i: i32| -> i32 {i + 1};
  let closure_inferred = |i | i + 1;

  let i = 1;
  // Calling the function and closures.
  println!("function: {}", function(i));
  println!("closure annotated: {}", closure_annotated(i));
  println!("closure inferred: {}", closure_inferred(i));

  // A closure taking no arguments which returns an `i32`.
  // The return type is inferred.
  let one = || 1;
  println!("closing returning one: {}", one());
}