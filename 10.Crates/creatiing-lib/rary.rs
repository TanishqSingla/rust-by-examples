pub fn public_function() {
  println!("called rary's `public_function()`");
}

pub fn private_function() {
  println!("called rary's `private_function()`");
}

pub fn indirect_access() {
  print!("called rary's `indirect_access()`, that\n> ");

  private_function();
}