// `as` can be used to give different names in import
use deeply::nested::function as other_function;

fn function() {
  println!("called `function()`");
}

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called `deeply::nested::function()`");
    }
  }
}

fn main() {
  other_function();

println!("Entering block");
  {
    // This is equivalent to `use::deeply::nested::function as function`
    // This function() will shoadow the outer one.
    use crate::deeply::nested::function;

    // `use` bindings have local scope
    function();

    println!("Leaving Block");
  }

  function();
}