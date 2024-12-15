use std::io; // Import the input/output module to read user input.

fn main() {
    println!("Temperature Converter"); // Display the purpose of the program.
    println!("Enter the temperature (e.g., 32F or 0C):"); // Ask the user to input a temperature.

    let mut input = String::new(); // Create a mutable String to hold user input.
    io::stdin()
        .read_line(&mut input) // Read a line of input from the user.
        .expect("Failed to read input"); // Handle errors if reading input fails.

    let input = input.trim(); // Trim whitespace from the input string.

    // Check if the input ends with "F" (Fahrenheit).
    if let Some(temp) = input.strip_suffix("F") {
        let fahrenheit: f64 = temp.parse().expect("Invalid number"); // Convert the numeric part to a floating-point number.
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0; // Convert Fahrenheit to Celsius using the formula.
        println!("{:.2}F is {:.2}C", fahrenheit, celsius); // Print the converted value.
    }
    // Check if the input ends with "C" (Celsius).
    else if let Some(temp) = input.strip_suffix("C") {
        let celsius: f64 = temp.parse().expect("Invalid number"); // Convert the numeric part to a floating-point number.
        let fahrenheit = celsius * 9.0 / 5.0 + 32.0; // Convert Celsius to Fahrenheit using the formula.
        println!("{:.2}C is {:.2}F", celsius, fahrenheit); // Print the converted value.
    }
    // If the input doesn't match the expected format, print an error message.
    else {
        println!("Invalid input format. Use '32F' or '0C'.");
    }
}
