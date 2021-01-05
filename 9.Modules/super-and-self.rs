fn function() {
  println!("called `function()`");
}

mod cool {
  pub fn function() {
    println!("called `cool::function()`");
  }
}

mod my {
  fn function() {
    print!("called `my::function()()")
  }

  mod cool {
    pub fn function() {
      println!("called `my::cool::function()`");
    }
  }

  pub fn indirect_call() {
    // Accessing all the functions named `function` from this scope!
    print!("called `my::indirect_call()`, that\n> ");

    // The `self` keyword refers to the current module scope - In this case `my`.
    // `self::function()` and `function()` are the same thing as they refer to the same function
    self::function();
    function();

    // We can use `self` to access other module inside `my`:
    self::cool::function();

    // The `super` keyword refers to the parent scope (outside the `my` module).
    super::function();

    // This will bind to the `cool::function()` in the *crate* scope.
    // In this case the crate scope is the outermost scope.
    {
      use crate::cool::function as root_function;
      root_function();
    }
  }
}

fn main() {
  my::indirect_call();
}