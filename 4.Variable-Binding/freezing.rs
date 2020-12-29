fn main() {
  let mut _mutable_integer = 7i32;
  {
    // shadowing by immutable `_mutable_integer`
    let _mutable_integer = _mutable_integer;
    
    // Error! `mutable_integer` is frozen in this scope
    _mutable_integer = 50;
    // Fixme ^ Comment out this line

    // _mutable_integer goes out of scope
  }
  // _mutable_integer is not frozen in this scope
  _mutable_integer = 3;
}