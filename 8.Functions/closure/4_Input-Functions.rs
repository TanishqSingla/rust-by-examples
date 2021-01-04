fn call_me<F: Fn()>(f: F) {
  f();
}


// A function satisfying `Fn` bound
fn function() {
  println!("I'm a function");
}

fn main() {
  // Closure satisfying `Fn` bound
  let closure = || println!("I'm a closure");

  call_me(closure);
  call_me(function);
}