// Suppress all the warning from casts which overflow
#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321_f32;

  // Error! No implicit type conversion
  // let integer: u8 = decimal;

  // Explicit conversion
  let integer = decimal as u8;
  let character = integer as char;

  // Error! There are limitations in conversion rules. A float cannot be directly converted to char.
  // let character = decimal as char;

  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  // When casting any value to unsigned type, T,
  // T::MAX + 1 is added or subtracted until the value
  // fits into the type
  
  // 1000 already fits in u16
  println!("1000 as a u16 is: {}", 1000 as u16);

  // 1000 - 256 - 256 = 232
  // Under the hood, the first 8 least significant bits (LSB) are kept
  // while the rest towards the most significant bits (MSB) get truncated. 
  println!("1000 as u8 is: {}", 1000 as u8);
  // -1 + 256 = 255
  println!("-1 as u8 is: {}", (-1i8) as u8);

  // For positive numbers this is same as modulus
  println!("1000 mod 256 is: {}", 1000 % 256);

  // When casting to a signed type, the (bitwise) results is the same as 
  // first casting to the corresponding unsigned type. If the most significant
  // bit of the value is 1, then the value is negative.

  // Unless it already fits of course.
  println!("128 as i16 is {}", 128 as i16);
  // 128 as u8 -> 128, who's two's complement in eight bits is:
  println!("128 as a i8 is {}", 127 as i8);

  // repeating the example above
  // 1000 as u8 -> 232
  println!("1000 as u8 is: {}", 1000 as u8); 
  // and the two's complement of 232 is -24
  println!("232 as i8 is: {}", 232 as i8);
}