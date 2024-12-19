use std::fmt::Debug;

// Define a trait for data handling
trait DataHandler<T> {
    fn process(&self) -> &T;
    fn get_metadata(&self) -> &str;
}

// Complex struct with lifetimes and generic types
struct AdvancedData<'a, T, U>
where
    T: Debug + Clone,
    U: Debug,
{
    primary_data: T,
    secondary_data: U,
    metadata: &'a str,
    reference_data: &'a [T],
}

// Implementation with lifetime specifications
impl<'a, T, U> AdvancedData<'a, T, U>
where
    T: Debug + Clone,
    U: Debug,
{
    fn new(primary: T, secondary: U, metadata: &'a str, reference: &'a [T]) -> Self {
        AdvancedData {
            primary_data: primary,
            secondary_data: secondary,
            metadata,
            reference_data: reference,
        }
    }

    fn get_reference_data(&self) -> &[T] {
        self.reference_data
    }
}

// Implement DataHandler trait with lifetime constraints
impl<'a, T, U> DataHandler<T> for AdvancedData<'a, T, U>
where
    T: Debug + Clone,
    U: Debug,
{
    fn process(&self) -> &T {
        &self.primary_data
    }

    fn get_metadata(&self) -> &str {
        self.metadata
    }
}

// Function that takes generic types with different lifetimes
fn compare_data<'a, 'b, T, U>(
    data1: &'a AdvancedData<'b, T, U>,
    data2: &'a AdvancedData<'b, T, U>,
) -> bool
where
    T: Debug + Clone + PartialEq,
    U: Debug + PartialEq,
{
    data1.primary_data == data2.primary_data && data1.secondary_data == data2.secondary_data
}

fn main() {
    // Create reference data
    let reference_data = vec![1, 2, 3, 4, 5];

    // Create first data handler
    let data1 = AdvancedData::new(42, "Secondary", "First metadata", &reference_data);

    // Create second data handler with same types
    let data2 = AdvancedData::new(42, "Different", "Second metadata", &reference_data);

    // Use the DataHandler trait
    println!("Data1 metadata: {}", data1.get_metadata());
    println!("Data1 processed: {:?}", data1.process());
    println!("Reference data: {:?}", data1.get_reference_data());

    // Compare data
    println!("Data equal: {}", compare_data(&data1, &data2));
}
