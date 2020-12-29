// It is possible to declare variable bindings first, and initialize them later. However this form is seldom used
// as it may lead to use of unintialized variable;

fn main() {
  // Declare a variable binding
  let a_binding;
  {
    let x = 2;

    // Initialized the binding
    a_binding = x * x;
  }
  println!("a binding: {}", a_binding);

  let another_binding;

  // The line below shows the error when a declared variable is left unintialized
  // println!("another binding: {}", another_binding);

  another_binding = 1;
  pritnln!("another binding: {}", another_binding);
}