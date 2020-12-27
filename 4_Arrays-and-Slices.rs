use std::mem;

// This function borrows a slice
// Slices have type signature `&[T]` where `T` is type
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-sized array (type signature is superfluous)
    let xs: [i32: 5] = [1, 2, 3, 4, 5];
    
    // All elements can be initialized to the same value
    let ys: [i32: 500] = [0; 500];
    
    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len()` returns the count of number of elements in array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("arrays occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as sliced
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of array as a slice");
    analyze_slice(xs[1..4]);
    
    // Out of bound indexing causes compile error
    // Uncomment the line below to see the error 
    // println!("{}", xs[5]);
}