// This file shows you how to implement display for types that have a sequence of data in them

/* 
    Since write!() returns a fmt::Result, it only returns one value, to handle this rust uses `?` operator
    e.g
    write!(f, "{}", v)?;
*/

use std::fmt;
