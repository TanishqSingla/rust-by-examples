fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {
    // A tuple with bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                     -1i8, -2i16, -3i32, -4i32,
                      0.1f32, 0.2f64, 'a', true);
    
    // Values can be extracted using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuples_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuples of tuples: {:?}", tuples_of_tuples);

    // Note: long tuples cannot be printed
    // Example is in the comments below, uncomment them to see the result
    
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair: {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just and integer: {:?}", (5u32))

    // Tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}