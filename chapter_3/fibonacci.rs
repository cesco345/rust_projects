use std::io; // Import the input/output module to read user input.

fn main() {
    println!("Fibonacci Generator"); // Display the purpose of the program.
    println!("Enter the position (n) of the Fibonacci sequence:"); // Ask the user to input the position of the Fibonacci number.

    let mut input = String::new(); // Create a mutable String to hold user input.
    io::stdin()
        .read_line(&mut input) // Read a line of input from the user.
        .expect("Failed to read input"); // Handle errors if reading input fails.

    let n: usize = input.trim().parse().expect("Invalid number"); // Parse the input into an unsigned integer.

    let fib_number = fibonacci(n); // Call the `fibonacci` function to calculate the nth Fibonacci number.
    println!("The {}th Fibonacci number is {}", n, fib_number); // Print the result.
}

// Recursive function to calculate the nth Fibonacci number.
fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,                                   // Base case: the 0th Fibonacci number is 0.
        1 => 1,                                   // Base case: the 1st Fibonacci number is 1.
        _ => fibonacci(n - 1) + fibonacci(n - 2), // Recursive case: sum of the two preceding numbers.
    }
}
