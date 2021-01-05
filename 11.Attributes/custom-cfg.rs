// Some function like target os are implicitly provided by rustc
// but custom conditionals require `--cfg` flag

#[cfg(some_condition)]
fn conditional_function() {
  println!("condition_met");
}

fn main() {
  conditional_function();
}

/*
  Command line argument passed: 
  rustc --cfg, some_condition custom-cfg.rs && ./custom-cfg.rs
*/