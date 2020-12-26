use std::fmt;


// defining a structure on which we will implement fmt::display
struct Structure(i32);

// Implementing fmt::display for Structure
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0);
    }
}