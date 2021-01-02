// Match keyword is similar to switch in `C` and can be used like a `switch` in C
fn main() {
  let number = 13;

  println!("Tell me about {}", number);
  match number {
    // match a single value
    1 => println!("One!"),
    // match several values
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    // Match an inclusive range
    13..=19 => println!("A teen"),
    // Handle the rest of cases
    _ => println!("Ain't special"),
  }

  let boolean = true;
  // Match is an expression too
  let binary = match boolean {
    // The arms of a match must cover all possible values
    false => 0,
    true => 1,
    // Try commenting one of the arms
  };

  println!("{} -> {}", boolean, binary);
}