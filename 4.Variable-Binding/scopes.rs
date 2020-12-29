fn main() {
  // This binding lives in this main function
  let long_lived_binding = 1;
  
  // This is a block, and has a smaller scope than the main function
  {
    // This binding only exists in this block
    let short_lived_binding = 2;

    println!("inner short: {}", short_lived_binding);
  }
  // End of block

  // The line below will throw an error since the variable doesn't exist in this scope
  // pritnln!("outer short: {}", short_lived_binding);

  println!("outer long: {}", long_lived_binding);
}