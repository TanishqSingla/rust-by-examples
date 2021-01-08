fn main() {
  #[derive(Debug)]
  struct Person {
    name: String,
    age: u8,
  }

  let person = Person {
    name: String::from("Alice"),
    age: 20,
  };

  // `name` is moved, but age is referenced
  let Person {name, ref age} = person;

  println!("The person's age is {}", age);
  println!("The person's name is {}", name);

  // Error! borrow of partially moved value: `person` partial moves occur
  // println!("The person struct is {:?}", person);

  // but person.age can be used
  println!("The person age from person struct is {}", person.age);
}