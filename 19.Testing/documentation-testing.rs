/// First line is a short summary describing function
/// 
/// The next line present detailed documentation. Code block starts with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
/// ```
/// let result = doccoments::add(2, 3);
/// assert_eq!(result, 5);
/// ```

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failiure".
/// 
/// The next function divides two numbers.
/// 
/// # Examples
/// 
/// ```
/// let result = doccomments::div(10, 2);
/// assert_eq(result, 5);
/// ```
/// 
/// # Panics
/// 
/// The function panics if the second argument is zero.
/// 
/// ```rust,should_panic
/// // panics on division by zero
/// doccomments::div(10, 0);
/// ```

pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divid-by-zero error");
    }

    a/b
}


/* Motivation behind documentation */
/// Using hidden `try_main` in doc tests.
/// 
/// ```
/// # // hidden lines start with `#` symbol, but they're still compatible complieable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = try::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #   try_main().unwrap(); // calling try_main and unwrapping so that test will panic in case of error
/// #}
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}