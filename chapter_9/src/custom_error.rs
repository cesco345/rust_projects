use std::error::Error;
use std::fmt;

// Custom error type for parsing numbers
#[derive(Debug)]
enum NumberError {
    Empty,
    Invalid(String),
    OutOfRange(i32),
}

// Implement Display trait for our error type
impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumberError::Empty => write!(f, "input string is empty"),
            NumberError::Invalid(s) => write!(f, "invalid number format: {}", s),
            NumberError::OutOfRange(n) => write!(f, "number {} is out of allowed range", n),
        }
    }
}

// Implement Error trait for our custom error type
impl Error for NumberError {}

// Function to parse and validate a number between 1 and 100
fn parse_number(s: &str) -> Result<i32, NumberError> {
    // Check for empty input
    if s.is_empty() {
        return Err(NumberError::Empty);
    }

    // Parse the string to a number
    let num = s
        .parse::<i32>()
        .map_err(|_| NumberError::Invalid(s.to_string()))?;

    // Validate range
    if num < 1 || num > 100 {
        return Err(NumberError::OutOfRange(num));
    }

    Ok(num)
}

fn main() {
    // Test various inputs
    let test_inputs = vec!["42", "", "abc", "0", "101"];

    for input in test_inputs {
        println!("Parsing '{}': {:?}", input, parse_number(input));
    }
}
