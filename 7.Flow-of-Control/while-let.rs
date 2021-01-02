#[allow(dead_code)]

fn _unused_code() {
  let mut optional = Some(0);

  // Repeatedly try this test
  loop {
    match optional {
      // If `optional` destructures, evaluate the block.
      Some(i) => {
        if i > 9 {
          println!("Greater than 9, quit!");
          optional = None;
        } else {
          println!("`i` is `{:?}`. Try again.", i);
          optional = Some(i + 1);
        }
      },
      _ => {break;}
    }
  }
}

fn main() {
  let mut optional = Some(0);

  while let Some(i) = optional {
    if i > 9 {
      println!("Greater than 9, quit!");
      optional = None;
    } else {
      println!("`i` is `{:?}`. Try again!", i);
      optional = Some(i + i);
    }
  }
}