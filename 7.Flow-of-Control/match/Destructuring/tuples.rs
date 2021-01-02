fn main() {
  let triple = (0, -2, 3);

  println!("Tell me about {:?}", triple);
  
  match triple {
    // Destructuring the second and third element
    (0, y, z) => println!("First is `0`, y is {:?}, and `z` is {:?}", y, z),
    (1, ..) => println!("First is `1` and the rest doesn't matter"),
    // `..` can be used to ignore the rest of tuple
    _ => println!("It doesn't matter what they are"),
    // `_` means don't bind any value to a variable 
  }
}