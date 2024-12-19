fn main() {
    // Test cases
    let test_strings = vec!["first", "apple", "hello", "world", "string"];

    // Convert and print each test case
    for word in test_strings {
        println!("Original: {}", word);
        println!("Pig Latin: {}\n", to_pig_latin(word));
    }
}

// Converts a single word to pig latin
fn to_pig_latin(word: &str) -> String {
    // Handle empty strings
    if word.is_empty() {
        return String::new();
    }

    // Get the first character and ensure it's valid UTF-8
    let first_char = word.chars().next().unwrap();

    // Check if the first character is a vowel
    if is_vowel(first_char) {
        // For words beginning with a vowel, add "hay"
        format!("{}-hay", word)
    } else {
        // For words beginning with a consonant:
        // 1. Take all characters after the first
        // 2. Add the first character and "ay" at the end
        let mut chars = word.chars();
        chars.next(); // Skip first character
        format!("{}-{first_char}ay", chars.collect::<String>())
    }
}

// Checks if a character is a vowel
fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}
