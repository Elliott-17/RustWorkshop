// TODO: Use `if let` to add 1 to the number if it exists.
// If None, return None.

fn add_one(input: Option<i32>) -> Option<i32> {
    // your code here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some() {
        assert_eq!(add_one(Some(1)), Some(2));
    }

    #[test]
    fn none() {
        assert_eq!(add_one(None), None);
    }
}