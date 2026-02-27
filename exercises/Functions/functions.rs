fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // Call the function
}

// Ignore Everything below this line
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, add(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, add(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, add(42, 42));
    }
}