use std::collections::HashMap;

fn main() {
    // Simulate a config file with optional settings
    let mut settings = HashMap::new();
    settings.insert("max_connections", Some(100));
    settings.insert("timeout", Some(30));
    settings.insert("debug_mode", None);

    // Read and apply settings using if let
    if let Some(max_conn) = settings.get("max_connections").unwrap_or(&None) {
        println!("Maximum connections set to: {}", max_conn);
    } else {
        println!("Using default maximum connections");
    }

    if let Some(timeout) = settings.get("timeout").unwrap_or(&None) {
        println!("Timeout set to: {} seconds", timeout);
    }

    // Demonstrate else usage
    if let Some(debug) = settings.get("debug_mode").unwrap_or(&None) {
        println!("Debug mode: {}", debug);
    } else {
        println!("Debug mode not configured, defaulting to false");
    }
}
