fn main() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  // copying an integer
  let copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit value: {:?}", unit);

  // The compiler throws a warning if a varible is not being used
  // To silence the warning simply add `_` as a prefix to variable name
  let _unused_variable = 3u32;
}