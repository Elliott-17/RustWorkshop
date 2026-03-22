// TODO: Complete the function using `if let`.
// Return the inner value if it exists,
// otherwise return 0.
// Do not use unwrap()
// Do not use match()

fn value_or_zero(input: Option<i32>) -> i32 {
    // your code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_value() {
        assert_eq!(value_or_zero(Some(7)), 7);
    }

    #[test]
    fn none_value() {
        assert_eq!(value_or_zero(None), 0);
    }
}