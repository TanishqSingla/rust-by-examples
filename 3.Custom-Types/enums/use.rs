#![allow(dead_code)]

// The `use` declaration can be used so that manual scoping isn't needed
// This files shows you how to use the `use` declaration

enum Status {
    Rich,
    Poor, 
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without manual scoping
    use crate::Status::{Poor, Rich};    
    use crate::Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

}