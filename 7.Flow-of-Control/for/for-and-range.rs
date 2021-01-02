// `..` in rust allows you to specify a range in rust, the range is in set [a, b)
// `..=` syntax can be used to make inclusive range on both ends, i.e [a, b]
fn main() {
  // `n` will take values from an inclusinve range of 1 and 100
  for n in 1..101 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }

  // The same program can be written as
  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }
}