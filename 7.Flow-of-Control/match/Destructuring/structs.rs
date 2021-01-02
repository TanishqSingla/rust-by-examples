fn main() {
  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  let foo = Foo {x: (1, 2), y: 2};

  match foo {
    Foo {x: (0, b), y} => println!("First of x is 1, b = {}, y = {} ", b, y),

    // You can destructure structs and rename variables
    // the order is not important
    Foo {y: 2, x: i} => println!("y is 2, i = {:?}", i),

    // and you can also ignore some variables:
    Foo {y, ..} => println!("y = {}, we don't care about x", y),
    // The line below will give you an error since pattern doesn not mention the field `x`
    // Foo { y } => println!("y = {}, y"),
  }
}