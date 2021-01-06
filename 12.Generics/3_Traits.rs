// Non-copyable types
struct Empty;
struct Null;

// A trait generic over 'T'.
trait DoubleDrop<T> {
  fn double_drop(self, _: T);
}

// Implementint double_drop for any generic parameter `T` and caller `U`
impl <T, U> DoubleDrop<T> for U {
  fn double_drop(self, _: T) {}
}

fn main() {
  let empty = Empty;
  let null = Null;

  // Deallocate `empty` and `null`
  empty.double_drop(null);

  //Error!
  // empty;
  // null; 
}