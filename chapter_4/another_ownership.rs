fn main() {
    let original_owner = String::from("Rust Ownership"); // `String` allocated on the heap and owned by `original_owner`.

    println!("Before transferring: {}", original_owner); // `original_owner` is valid here.

    let new_owner = take_ownership(original_owner); // Ownership of the `String` is transferred to the `take_ownership` function.

    // Uncommenting the next line will cause a compile-time error because `original_owner` is no longer valid.
    // println!("{}", original_owner);

    println!("After transferring, new owner: {}", new_owner); // `new_owner` now owns the string.

    let length = borrow_ownership(&new_owner); // Borrow the ownership by passing a reference to `new_owner`.

    println!(
        "The string '{}' has a length of {} characters.",
        new_owner, length
    ); // `new_owner` is still valid after borrowing.
}

// Function to take ownership of a `String`.
fn take_ownership(s: String) -> String {
    println!("Taking ownership of: {}", s); // Print the string to show we have ownership.
    s // Return ownership back to the caller.
}

// Function to borrow ownership and calculate the length of the string.
fn borrow_ownership(s: &String) -> usize {
    println!("Borrowing ownership of: {}", s); // We can read the string, but we cannot modify it.
    s.len() // Return the length of the string.
}
