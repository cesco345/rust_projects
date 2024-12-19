use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};

// Custom error type for network operations
#[derive(Debug)]
enum NetworkError {
    Connection(String),
    Timeout(u32),
    IoError(io::Error),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NetworkError::Connection(msg) => write!(f, "Connection error: {}", msg),
            NetworkError::Timeout(secs) => write!(f, "Operation timed out after {} seconds", secs),
            NetworkError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for NetworkError {}

// Simulation of a network operation that might fail
fn download_data(url: &str, save_to: &str) -> Result<(), NetworkError> {
    // Simulate connection error for certain URLs
    if url.contains("invalid") {
        return Err(NetworkError::Connection("Invalid host".to_string()));
    }

    // Simulate timeout for long operations
    if url.contains("slow") {
        return Err(NetworkError::Timeout(30));
    }

    // Attempt to create a file to save the data
    let mut file = File::create(save_to).map_err(NetworkError::IoError)?;

    // Simulate writing downloaded data
    file.write_all(b"Downloaded content")
        .map_err(NetworkError::IoError)?;

    Ok(())
}

fn main() {
    // Test different scenarios
    let tests = vec![
        ("http://example.com/file", "good.txt"),
        ("http://invalid.com/file", "bad.txt"),
        ("http://slow.example.com/file", "slow.txt"),
    ];

    for (url, file) in tests {
        println!("Downloading {} to {}", url, file);
        match download_data(url, file) {
            Ok(()) => println!("Success!"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
