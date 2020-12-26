// This file shows you how to implement display for types that have a sequence of data in them

/* 
    Since write!() returns a fmt::Result, it only returns one value, to handle this rust uses `?` operator
    e.g
    write!(f, "{}", v)?;
*/

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for(count, v) in vec.iter().enumerate() {
            if count != 0 {write!(f, ", ")?;}

            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);

    println!("{}", v);
}