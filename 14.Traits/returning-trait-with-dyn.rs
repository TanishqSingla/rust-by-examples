struct Sheep {}
struct Cow {}

trait Animal {
  // Instance method signature
  fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "baaaah!"
  }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "moooooo!"
  }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

fn main() {
  let random_number = 0.234;
  let animal = random_animal(random_number);
  println!("You've radomly chosen an animal, and it says {}", animal.noise());
}