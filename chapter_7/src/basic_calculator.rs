// File: src/main.rs
// This is a simple binary crate that demonstrates the basic structure
// of a Rust package with a single binary

// Bring in the standard library's IO module for basic input/output
use std::io;

// Main function - the entry point of a binary crate
// Every binary crate must have a main function
fn main() {
    println!("Welcome to the Basic Calculator!");

    // Get first number from user
    println!("Enter first number:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    // Parse string to number, providing error handling
    let num1: f64 = match num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Get second number using the same process
    println!("Enter second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num2: f64 = match num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Perform and display basic calculations
    println!("Sum: {}", num1 + num2);
    println!("Difference: {}", num1 - num2);
    println!("Product: {}", num1 * num2);

    // Division needs special handling for division by zero
    if num2 != 0.0 {
        println!("Division: {}", num1 / num2);
    } else {
        println!("Cannot divide by zero!");
    }
}
