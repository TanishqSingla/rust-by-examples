// For some use cases, when matching enums, `match` is awkward. For example
#[allow(dead_code)]

fn _unused_example() {
  let optional = Some(7);

  match optional {
    Some(i) => {
      println!("This is a really long string and `{:?}`", i);
      // Needed 2 indentation to destructure `i` from option
    },
    _ => {},
    // ^ Required because `match` is exaustive. Doesn't it seem
    // like waste of space
  }
}

// if let cleaner for this use and in addition allows various failiure options to be specified

fn _example_main() {
  // ALl have type `Option<i32>`
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  // The `if let` construct reads: "if `let` destructures `number` into Some(i), evaluate the block (`{}`)."
  if let Some(i) = number {
    println!("Matched: {:?}!", i);
  }

  // if you need to specify a failire, use an else
  if let Some(i) = letter {
    println!("Matched: {:?}!", i);
  } else {
    // Destructure failed, change to failiure case. 
    println!("Didn't match a number. Let's go with a letter");
  }

  // provided an altered failing condition
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched: {:?}", i);
  } else if i_like_letters {
    println!("Didn't match a number, Let's go with a letter!");
  } else {
    println!("I don't like letters, Let's go with an emoticon");
  }
}

// Our example enum
enum Foo {
  Bar,
  Baz,
  Qux(u32),
}

fn main() {
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  // Variable a matches Foo::Bar
  if let Foo::Bar = a {
    println!("a is a foobar");
  }

  // Variable b doesn't matches so it will not print
  if let Foo::Bar = b {
    println!("b is foobar");
  }

  // Variable c matches Foo::Qux which has a value
  // Similar to Some() in the previous example
  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  // Binding also works with `if let`
  if let Foo::Qux(value @ 100) = c {
    println!("c is {}", value);
  }
}