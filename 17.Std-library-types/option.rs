// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in `Some()` variant
        Some(dividend / divisor)
    }
}

// This functions checks division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be a type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_flaot = Some(0f32);

    // Unwrapping the `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_flaot, optional_flaot.unwrap());

    // Unwrapping a `None` variant will panic
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}