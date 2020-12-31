fn main() {
  let n = 5;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  // rust also allows this
  let big_n = if n < 10 && n > -10 {
    println!(", and is a small number, increase ten-fold");

    // This expression returns an `i32`
    10 * n
  } else {
    println!(", and is a big number, halve the number");

    // This expression returns an i32 as well
    n / 2
  };
  // Don't forget this semicolon

  println!("{} -> {}", n, big_n);
}