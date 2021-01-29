pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// The purpose of this function is to fail
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
}