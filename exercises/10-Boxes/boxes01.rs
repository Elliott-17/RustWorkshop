// 1. Use Box<dyn Error> for flexible error handling
// 2. Use Box<dyn Trait> in vectors to store different types

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DivideByZeroError;

impl fmt::Display for DivideByZeroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot divide by zero")
    }
}

impl Error for DivideByZeroError {}



#[derive(Debug)]
pub struct InvalidInputError(String);

impl fmt::Display for InvalidInputError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Invalid input: {}", self.0)
        }
}

impl Error for InvalidInputError {}



// TODO: Complete this function to return Box<dyn Error>
// It should divide two numbers and return an error if dividing by zero
pub fn divide(a: f64, b: f64) -> Result<f64, Box<dyn Error>> {
        // TODO
}

// TODO: Complete this function to parse and multiply two numbers
// Return Box<dyn Error> if parsing fails
pub fn parse_and_multiply(a: &str, b: &str) -> Result<f64, Box<dyn Error>> {
        // TODO
}

// A simple trait for operations
pub trait Operation {
        fn execute(&self) -> f64;
}

struct AddOp(f64, f64);
impl Operation for AddOp {
        fn execute(&self) -> f64 {
                self.0 + self.1
        }
}

struct MultiplyOp(f64, f64);
impl Operation for MultiplyOp {
        fn execute(&self) -> f64 {
                self.0 * self.1
        }
}

// TODO: Complete this function
// Create a Vec of Box<dyn Operation> containing different operation types
// Then execute all operations and return the sum of results
// Implement the following operations:
//      5.0 + 3.0
//      4.0 * 2.0
//      10.0 + 5.0
pub fn execute_operations() -> f64 {
        // TODO
}

fn main() {
        // Test divide function
        let result1 = divide(10.0, 2.0).unwrap();
        assert_eq!(result1, 5.0);

        // Test divide by zero
        let result2 = divide(10.0, 0.0);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err().to_string(), "Cannot divide by zero");

        // Test parse and multiply
        let result3 = parse_and_multiply("5", "3").unwrap();
        assert_eq!(result3, 15.0);
        
        // Test invalid input
        let result4 = parse_and_multiply("hello", "5");
        assert!(result4.is_err());

        // Test operations
        let result5 = execute_operations();
        assert_eq!(result5, 31.0);
}
