// A contact management system using HashMap and String manipulation
use std::collections::HashMap;

struct Contact {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    // Initialize contact storage using HashMap
    let mut contacts: HashMap<String, Contact> = HashMap::new();

    // Vector of tuples containing test data
    let contact_data = vec![
        ("Alice", "123-456-7890", "alice@email.com"),
        ("Bob", "234-567-8901", "bob@email.com"),
        ("Charlie", "345-678-9012", "charlie@email.com"),
    ];

    // Insert contacts into HashMap
    for (name, phone, email) in contact_data {
        contacts.insert(
            name.to_lowercase(), // Use lowercase key for case-insensitive lookup
            Contact {
                name: String::from(name),
                phone: String::from(phone),
                email: String::from(email),
            },
        );
    }

    // Demonstrate contact lookup (case-insensitive)
    let search_name = "ALICE";
    if let Some(contact) = contacts.get(&search_name.to_lowercase()) {
        println!("Found contact:");
        println!("Name: {}", contact.name);
        println!("Phone: {}", contact.phone);
        println!("Email: {}", contact.email);
    }

    // Print all contacts in alphabetical order
    println!("\nAll Contacts:");
    let mut names: Vec<&String> = contacts.values().map(|c| &c.name).collect();
    names.sort();
    for name in names {
        if let Some(contact) = contacts.get(&name.to_lowercase()) {
            println!("{} - {} - {}", contact.name, contact.phone, contact.email);
        }
    }
}
