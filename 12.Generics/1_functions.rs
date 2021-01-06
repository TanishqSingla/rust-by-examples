struct A;         // Concrete type `A`
struct S(A);      // Concrete type `S`
struct SGen<T>(T);// generic type `SGen`

// Defining a concrete type function
fn reg_fn(_s: S) {}

// This function takes argument SGen which has been implicitly provided type parameter A.
// Hence this function is not generic
fn gen_spec_t(_s: SGen<A>) {}

// This function also has an i32 as a type parameter in SGen, so this funcion is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// This function is generic over T
fn generic<T>(_s: SGen<T>) {}

fn main() {
  // Using the non-generic functions
  reg_fn(S(A));
  gen_spec_t(SGen(A));
  gen_spec_i32(SGen(5));

  // Explicitly providing type parameter to function call `generic`
  generic::<char>(SGen('a'));

  // Implicitly passing type parameter to function call `generic`
  generic(SGen('c'));

}