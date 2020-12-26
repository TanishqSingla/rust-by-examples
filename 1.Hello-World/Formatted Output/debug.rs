#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure); 

fn main() {
    //Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
             "Slater", 
             "Christian", 
             actor="actor\'s");
    // `Structure` is printable
    println!("Now {:?} will print!", Structure(3));
}