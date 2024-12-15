fn main() {
    let sentence = String::from("Rust is a systems programming language."); // Create a `String`.

    let first_word = get_first_word(&sentence); // Extract the first word using a string slice.
    println!("The first word is: '{}'", first_word);

    let second_word = get_second_word(&sentence); // Extract the second word using a slice.
    println!("The second word is: '{}'", second_word);

    // Slices let us focus on a part of the string without copying the data.
    let partial_sentence = &sentence[0..11]; // A slice of the first 11 characters of the sentence.
    println!("Partial sentence: '{}'", partial_sentence);

    // The original string remains intact and can still be used.
    println!("The entire sentence: '{}'", sentence);
}

// Function to find the first word in a string slice.
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the string to a byte array for easy traversal.

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // Check for a space character.
            return &s[0..i]; // Return a slice from the start to the first space.
        }
    }

    &s[..] // If no space is found, return the entire string.
}

// Function to find the second word in a string slice.
fn get_second_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // Convert the string to a byte array.

    let mut space_found = false; // Flag to indicate the first space is found.

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            if space_found {
                // Return a slice starting from the second space.
                return &s[i + 1..];
            }
            space_found = true; // Mark that the first space was found.
        }
    }

    &s[..] // If no second word is found, return the entire string.
}
