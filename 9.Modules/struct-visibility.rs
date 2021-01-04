mod my {
  // A public struct with a public field of generic type `T`
  pub struct OpenBox<T> {
    pub contents: T,
  }

  // A public struct with a private field of generic type `T`
  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T,
  }

  impl<T> ClosedBox {
    // A public constructor method
    pub fn new(contents: T) -> ClosedBox {
      ClosedBox {
        contents: contents,
      }
    }
  }
}

fn main() {
  // Public struct with public fields can be constructed as usual
  let open_box = my::OpenBox { contents: "Public information" };

  // and their fields can be noramally accessed.
  println!("The open box contains: {}", open_box.contents);

  // Public sturcts with private fields cannot be constructed using field names.
  // Error! `ClosedBox` has private fields
  // let closed_box = my::ClosedBox {contents: "classified information"};

  // However, structs with private fields can be created using
  // public constructors
  let _closed_box = my::ClosedBox::new("classified information");

  // and the private fields of a public struct cannot be accessed.
  // Error! The `contents` field is private.
  // println!("The closed box contains: {}", _closed_box.contents);
}