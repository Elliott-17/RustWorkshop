
fn main() {
    println!("These functions have many many problems! Go through each one and fix the issues to make the tests pass.");

    let a = 5;
    let b = 10;

    let sum = add_numbers(a, b);
    println!("The sum of {a} and {b} is: {sum}");

    let double = double_number(a);
    println!("The double of {a} is: {double}");

    let max = get_maximum(a, b);
    println!("The maximum of {a} and {b} is: {max}");

    let even_check = is_even(a);
    println!("{a} is even: {even_check}");

    let width = 4;
    let height = 5;
    let area = rectangle_area(width, height);
    println!("The area of a rectangle with width {width} and height {height} is: {area}");
}

// TODO: This function should return the sum of a and b
pub fn add_numbers(a: integer, b: integer) -> integer {
    a * 3 - b
}

// TODO: This function should return the double of the input number
fn double_number(x: integer) -> char {
     x + 1
}

// TODO: This function should return the maximum of a and b
fn get_maximum(a: integer, b: integer) -> char {
    if a > b {
        a
    } else {
        b
    }
}

// TODO: This function should return true if the number is even, and false otherwise
fn is_even(number: integer) -> char {
    number % 2 == 0
}

// TODO: This function should return the area of a rectangle given its width and height
fn rectangle_area(width: unsigned_integer, height: unsigned_integer) -> unsigned_integer {
    width * height
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
        assert!(is_even(4));
        assert!(!is_even(7));
        assert!(is_even(0));
        assert!(is_even(-2));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(rectangle_area(3, 4), 12);
        assert_eq!(rectangle_area(5, 5), 25);
        assert_eq!(rectangle_area(1, 10), 10);
    }

}

