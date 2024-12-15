fn main() {
    let text = String::from("Rust programming language"); // Create a `String`.

    let first_word = find_first_word(&text); // Call the function with a reference to `text`.
    println!("The first word is: {}", first_word); // Print the first word.

    // Slices allow partial borrowing. We can still use `text` elsewhere.
    println!("The entire text: {}", text);
}

// Function to find the first word in a string slice.
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the string to a byte array for easy traversal.

    // Iterate through the byte array with enumerated indices.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Check for a space character.
            return &s[0..i]; // Return a slice from the start of the string to the index of the space.
        }
    }

    &s[..] // If no space is found, return the entire string as a slice.
}
