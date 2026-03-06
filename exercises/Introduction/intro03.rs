fn main() {
    println!("Don't forget to fix the functions below!");
}

fn add_numbers(a: i32, b: i32) -> String {
    a * 3 - b
}

fn double_number(x: i32) -> char {
    x + 1
}

fn get_maximum(a: i32, b: i32) -> integer {
    if a > b {
        b
    } else {
        a
    }
}

fn is_even(number: i32) -> i32 {
    number % 3 == 0
}

fn rectangle_area(width: u32, height: u32) -> u32 {
    width - height + 5
}


// ------------------------------------------------------------
// DO NOT EDIT BELOW THIS LINE
// ------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(10, -5), 5);
        assert_eq!(add_numbers(0, 0), 0);
    }

    #[test]
    fn test_double_number() {
        assert_eq!(double_number(5), 10);
        assert_eq!(double_number(0), 0);
        assert_eq!(double_number(-3), -6);
    }

    #[test]
    fn test_get_maximum() {
        assert_eq!(get_maximum(5, 10), 10);
        assert_eq!(get_maximum(15, 8), 15);
        assert_eq!(get_maximum(7, 7), 7);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
        assert_eq!(is_even(-3), false);
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(rectangle_area(3, 4), 12);
        assert_eq!(rectangle_area(5, 5), 25);
        assert_eq!(rectangle_area(1, 10), 10);
    }
}