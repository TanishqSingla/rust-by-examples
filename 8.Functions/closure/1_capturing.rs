fn main() {
  use std::mem;

  let color = String::from("green");

  // A closure to print `color` which immediately borrows (`&`) `color` and stores the borrow
  // and closure in the print variable. It will remain borrowed until `print` is used the last time.

  // println!() only requires arguments by immutable reference so it doesn't impose anything more
  // restrictive.
  let print = || println!("`color`: {}", color);

  // Call the closure using borrow
  print();

  // Color can be borrowed mutably again, because the closure only holds an immutable
  // reference to a `color`
  let _reborrow = &color;

  let mut count = 0;
  // A closure to incrememnt `count` could take either `&mut count` or `count`
  // but `&mut count` is less restrictive, so it takes that. Immediately borrows count

  // A `mut` is required on `inc` because an `&mut is stored inside`. Thus, calling the
  // closure which requires a `mut`.
  let mut inc = || {
    count += 1;
    println!("count: {}", count);
  };

  // Call the closure using mutable borrow
  inc();

  // The closure still mutably borrows `count` because it is called later.
  // An attempt to reborrow will lead to an error.
  // let _reborrow = &count;
  // ^ TODO try to uncomment this line
  inc();

  // The closure no longer needs to borrow `&mut count`. Therefore, it is
  // possible to reborrow without error
  let _count_reborrowed = &mut count;

  // A non-copy type.
  let movable = Box::new(3);
  // mem::drop requires `T`, so this must take by value. A copy type would copy into
  // the closure leaving the original untouched. A non-copy must move and so `movable`
  // immediately moves into the closure.
  let consume = || {
    println!("`movable`: {}", movable);
    mem::drop(movable);
  };

  // consume consumes the variable so this can be called only once. 
  consume();
  // consume();
  // Try uncommenting this line;

  // `Vec` has non-copy semantics
  let haystack = vec![1, 2, 3];

  let contains = move |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));
}