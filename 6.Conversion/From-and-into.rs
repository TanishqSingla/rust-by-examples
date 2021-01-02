use std::convert::From;

#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number {value: item} 
  }
}

fn main() {
  // general example of `from` trait
  // This example converts an `str` to a String
  let my_str = "hello";
  let _my_string = String::from(my_str);

  let num = Number::from(30);
  println!("My number is {:?}", num);

  let int = 5;
  let number: Number = int.into();
  println!("My Number is: {:?}", number);
}
