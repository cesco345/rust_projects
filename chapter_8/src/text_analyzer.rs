use std::collections::HashMap;

// Structure to hold text analysis results
struct TextStats {
    char_count: usize,
    word_count: usize,
    line_count: usize,
    word_frequencies: HashMap<String, usize>,
    char_frequencies: HashMap<char, usize>,
    average_word_length: f64,
}

fn analyze_text(text: &str) -> TextStats {
    // Initialize counters and storage
    let mut word_frequencies = HashMap::new();
    let mut char_frequencies = HashMap::new();
    let mut total_word_length = 0;

    // Count lines (add 1 if text doesn't end with newline but has content)
    let line_count = text.lines().count();

    // Process words
    let words: Vec<String> = text
        .split_whitespace()
        .map(|word| {
            // Clean word: convert to lowercase and remove punctuation
            word.trim_matches(|c: char| !c.is_alphabetic())
                .to_lowercase()
        })
        .filter(|word| !word.is_empty()) // Remove empty strings
        .collect();

    // Calculate word frequencies and total length
    for word in &words {
        *word_frequencies.entry(word.to_string()).or_insert(0) += 1;
        total_word_length += word.len();
    }

    // Count character frequencies (excluding whitespace)
    for c in text.chars().filter(|c| !c.is_whitespace()) {
        *char_frequencies.entry(c).or_insert(0) += 1;
    }

    // Calculate average word length
    let average_word_length = if !words.is_empty() {
        total_word_length as f64 / words.len() as f64
    } else {
        0.0
    };

    TextStats {
        char_count: text.chars().filter(|c| !c.is_whitespace()).count(),
        word_count: words.len(),
        line_count,
        word_frequencies,
        char_frequencies,
        average_word_length,
    }
}

fn main() {
    // Sample text for analysis
    let text = "Rust is a systems programming language.
It is designed to be safe, concurrent, and practical.
Rust is fast and memory-efficient.";

    // Perform analysis
    let stats = analyze_text(text);

    // Print results
    println!("Text Analysis Results:");
    println!("----------------------");
    println!("Character count: {}", stats.char_count);
    println!("Word count: {}", stats.word_count);
    println!("Line count: {}", stats.line_count);
    println!("Average word length: {:.2}", stats.average_word_length);

    println!("\nMost common words:");
    let mut word_freq: Vec<(&String, &usize)> = stats.word_frequencies.iter().collect();
    word_freq.sort_by(|a, b| b.1.cmp(a.1));
    for (word, count) in word_freq.iter().take(5) {
        println!("{}: {}", word, count);
    }

    println!("\nCharacter frequencies:");
    let mut char_freq: Vec<(&char, &usize)> = stats.char_frequencies.iter().collect();
    char_freq.sort_by(|a, b| b.1.cmp(a.1));
    for (char, count) in char_freq.iter().take(5) {
        println!("{}: {}", char, count);
    }
}
