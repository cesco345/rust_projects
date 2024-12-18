use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter a number (or 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Exit condition using if let
        if let "q" = input {
            println!("Goodbye!");
            break;
        }

        // Parse number using if let
        if let Ok(number) = input.parse::<i32>() {
            println!("You entered: {}", number);
            if let Some(doubled) = number.checked_mul(2) {
                println!("Double of your number is: {}", doubled);
            } else {
                println!("Couldn't double your number - too large!");
            }
        } else {
            println!("That's not a valid number!");
        }
    }
}
