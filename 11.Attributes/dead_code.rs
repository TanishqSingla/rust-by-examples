// dead_code is used to disable the ling warning about unused functions

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// ^FIXME add dead_code attribute


fn main() {
  unused_function();
}