use std::fmt::Display;

// Define a trait for data processing with additional functionality
trait Processor {
    fn process(&self) -> String;
    fn get_type(&self) -> &str;
    // Add a validation method to the trait
    fn is_valid(&self) -> bool;
}

// Generic structure for holding data
struct DataProcessor<T>
where
    T: Display,
{
    data: T,
    name: String,
    version: u32, // Added version field for demonstration
}

// Implementation for DataProcessor with trait bounds
impl<T> DataProcessor<T>
where
    T: Display,
{
    fn new(data: T, name: String) -> Self {
        DataProcessor {
            data,
            name,
            version: 1,
        }
    }

    fn get_data(&self) -> &T {
        &self.data
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn get_info(&self) -> String {
        format!("Processor: {} (v{})", self.name, self.version)
    }
}

// Implement Processor trait for DataProcessor
impl<T> Processor for DataProcessor<T>
where
    T: Display,
{
    fn process(&self) -> String {
        format!("Processed {} data: {}", self.name, self.data)
    }

    fn get_type(&self) -> &str {
        std::any::type_name::<T>()
    }

    fn is_valid(&self) -> bool {
        !self.name.is_empty() && self.version > 0
    }
}

// Function to demonstrate processing multiple types
fn process_data<T: Display>(processor: &DataProcessor<T>) {
    println!("Processing Data:");
    println!("---------------");
    println!("Info: {}", processor.get_info());
    println!("Raw data: {}", processor.get_data());
    println!("Processed: {}", processor.process());
    println!("Type: {}", processor.get_type());
    println!("Valid: {}", processor.is_valid());
    println!();
}

fn main() {
    // Test with numeric data
    println!("Numeric Data Test:");
    let mut num_processor = DataProcessor::new(42, String::from("numeric"));
    process_data(&num_processor);

    // Demonstrate version increment
    num_processor.increment_version();
    println!("After version increment:");
    println!("New {}", num_processor.get_info());
    println!();

    // Test with text data
    println!("Text Data Test:");
    let text_processor = DataProcessor::new(String::from("Hello, World!"), String::from("text"));
    process_data(&text_processor);

    // Test with floating point data
    println!("Float Data Test:");
    let float_processor = DataProcessor::new(3.14159, String::from("float"));
    process_data(&float_processor);

    // Test with an empty name (invalid case)
    println!("Invalid Processor Test:");
    let invalid_processor = DataProcessor::new(42, String::from(""));
    println!("Valid: {}", invalid_processor.is_valid());
}
