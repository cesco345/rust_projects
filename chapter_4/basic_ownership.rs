fn main() {
    let s1 = String::from("Hello, Rust!"); // Create a String `s1`. Ownership of the string is with `s1`.
    let s2 = s1; // Transfer ownership of the string to `s2`. `s1` is now invalid.

    // Uncommenting the next line will cause a compile-time error because `s1` no longer owns the string.
    // println!("{}", s1);

    println!("{}", s2); // `s2` now owns the string, so we can print it here.

    let s3 = s2.clone(); // Create a deep copy of the string. Both `s2` and `s3` now own independent copies.

    println!("s2: {}, s3: {}", s2, s3); // Both `s2` and `s3` can be used.
}
