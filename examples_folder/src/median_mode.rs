use std::collections::HashMap;

fn main() {
    // Test vector of integers
    let numbers = vec![
        1, 4, 6, 2, 4, 9, 7, 6, 3, 2, 7, 9, 6, 9, 3, 5, 4, 2, 6, 8, 1, 2, 4, 6, 8, 4,
    ];

    // Calculate and display results
    println!("Numbers: {:?}", numbers);
    println!("Median: {}", find_median(&numbers));
    println!("Mode: {}", find_mode(&numbers));
}

// Calculates the median value of a vector of integers
fn find_median(numbers: &Vec<i32>) -> f64 {
    // Create a mutable copy of the vector for sorting
    let mut sorted = numbers.clone();
    sorted.sort();

    let len = sorted.len();

    // Handle empty vector case
    if len == 0 {
        return 0.0;
    }

    // If length is odd, return middle element
    // If even, return average of two middle elements
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (sorted[mid_left] + sorted[mid_right]) as f64 / 2.0
    } else {
        sorted[len / 2] as f64
    }
}

// Finds the mode (most frequent value) in a vector of integers
fn find_mode(numbers: &Vec<i32>) -> i32 {
    // Create a HashMap to store frequency counts
    let mut frequency_map = HashMap::new();

    // Count occurrences of each number
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Find the number with highest frequency
    frequency_map
        .into_iter()
        .max_by_key(|&(_, count)| count) // Compare by count
        .map(|(val, _)| val) // Extract the value
        .unwrap_or(0) // Default to 0 if vector is empty
}
