// A program that performs basic word statistics on a sentence
use std::collections::HashMap;

fn main() {
    // Input text to analyze
    let text = "rust is a great language rust is fast and reliable";

    // Initialize a vector to store individual words
    let mut words: Vec<String> = text
        .split_whitespace() // Split text into words
        .map(String::from) // Convert each word to owned String
        .collect(); // Collect into vector

    // Sort words for consistent output
    words.sort();

    // Create a HashMap to store word frequencies
    let mut word_counts = HashMap::new();

    // Count occurrences of each word
    for word in &words {
        // Entry API provides a way to insert or modify value based on presence
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    // Print statistics
    println!("Word Statistics:");
    println!("Total words: {}", words.len());
    println!("Unique words: {}", word_counts.len());
    println!("\nWord frequencies:");
    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
}
