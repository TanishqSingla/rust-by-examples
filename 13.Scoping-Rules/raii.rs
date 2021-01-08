fn create_box() {
  // Allocate an integer on the heap
  let _box1 = Box::new(3i32);

  // `_box1` gets destroyed here, and memory gets freed
}

fn main() {
  // Allocate an integer on the heap
  let _box2 = Box::new(5i32);

  // A nested scope
  {
    let _box3 = Box::new(4i32);
  }

  create_box(); 

  // _box 2 gets destroyed here
}