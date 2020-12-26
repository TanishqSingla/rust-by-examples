use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    // Printing using debug trait
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    //Printing using display
    println!("{}", matrix);

    // Printing transpose
    println!("Transpose:\n{}", transpose(matrix));
}