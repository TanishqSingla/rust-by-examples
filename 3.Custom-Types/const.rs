// Declaring globals
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  // Accessing the constant
  n > THRESHOLD
}

fn main() {
  let n = 16;

  // Access constant in the main thread
  println!("This is {}", LANGUAGE);
  println!("The threshold is {}", THRESHOLD);
  println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

  // The line below will give an error, to see the error uncomment the line below
  // THRESHOLD = 5;

}