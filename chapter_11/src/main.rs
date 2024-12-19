use chapter_11::{Calculator, StringUtils, UserManager};

fn main() {
    // Calculator demo
    println!("Calculator Demo:");
    let mut calc = Calculator::new();
    println!("2 + 2 = {}", calc.add(2.0, 2.0));
    println!("10 - 4 = {}", calc.subtract(10.0, 4.0));
    println!();

    // String utils demo
    println!("String Utils Demo:");
    let utils = StringUtils::new(false);
    let text = "The quick brown fox jumps over the lazy dog";
    println!(
        "Occurrences of 'the': {}",
        utils.count_occurrences(text, "the")
    );
    println!(
        "Is valid email: {}",
        utils.is_valid_email("test@example.com")
    );
    println!();

    // User manager demo
    println!("User Manager Demo:");
    let mut manager = UserManager::new();
    match manager.create_user("testuser", "test@example.com") {
        Ok(user) => println!("Created user: {:?}", user),
        Err(e) => println!("Error creating user: {:?}", e),
    }
}
