use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

// Function to read a file's contents with explicit error handling
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    // First check if file exists to provide a more specific error message
    if !Path::new(path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{}' does not exist", path),
        ));
    }

    // Attempt to open the file
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Create a string to store the content
    let mut content = String::new();

    // Read the file content into the string
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

fn main() {
    // Test with various file scenarios
    let files = vec!["existing.txt", "missing.txt", "invalid_utf8.bin"];

    for file_path in files {
        println!("Attempting to read: {}", file_path);
        match read_file_contents(file_path) {
            Ok(content) => println!("Content: {}", content),
            Err(error) => println!("Error: {}", error),
        }
    }
}
