// Define different types of payment methods that can contain associated data
#[derive(Debug)]
enum PaymentMethod {
    Cash(f64),               // Stores just the amount
    CreditCard(String, f64), // Stores card number and amount
    PayPal(String, f64),     // Stores email and amount
}

// Function to process payments based on their type
fn process_payment(payment: PaymentMethod) {
    // Use match to handle each type of payment differently
    match payment {
        // For cash payments, no additional fees
        PaymentMethod::Cash(amount) => {
            println!("Processing cash payment of ${:.2}", amount);
            println!("No additional fees!");
        }
        // Credit card payments have a 3% processing fee
        PaymentMethod::CreditCard(number, amount) => {
            let fee = amount * 0.03; // Calculate 3% fee
            println!("Processing credit card payment of ${:.2}", amount);
            // Only show last 4 digits of card for security
            println!("Card ending in: {}...", &number[number.len() - 4..]);
            println!("Processing fee: ${:.2}", fee);
            println!("Total charge: ${:.2}", amount + fee);
        }
        // PayPal has a 2.9% + $0.30 fee structure
        PaymentMethod::PayPal(email, amount) => {
            let fee = amount * 0.029 + 0.30; // Calculate PayPal's fee
            println!("Processing PayPal payment of ${:.2}", amount);
            println!("PayPal account: {}", email);
            println!("Processing fee: ${:.2}", fee);
            println!("Total charge: ${:.2}", amount + fee);
        }
    }
}

fn main() {
    // Create a vector of different payment types to test
    let payments = vec![
        PaymentMethod::Cash(50.0),
        PaymentMethod::CreditCard(String::from("4532756279624591"), 100.0),
        PaymentMethod::PayPal(String::from("user@example.com"), 75.0),
    ];

    // Process each payment in the vector
    for payment in payments {
        process_payment(payment);
        println!("------------------------"); // Separator for readability
    }
}
