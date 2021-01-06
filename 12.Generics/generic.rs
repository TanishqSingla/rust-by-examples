// This is a concrete type
struct A;

// This is a generic type
struct SingleGen<T>(T);

fn main() {
  let _char: SingleGen<char> = SingleGen('A');

  // Single gen can also have type implicitly
  let _t = SingleGen(A);
  let _i32 = SingleGen(6);
  let _chhar SingleGen('a'); // Uses `char`
}