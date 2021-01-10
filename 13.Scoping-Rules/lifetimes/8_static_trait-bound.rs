use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
  println!("static value passed in is: {:?}", input);
}

fn use_it() {
  // i is owned and contains no references, thus it's 'static:
  let i = 5;
  print_it(i);

  // oops &i only has the lifetime defined by the scope of use_it(), so it's not 'static:
  print_it(&i);
}