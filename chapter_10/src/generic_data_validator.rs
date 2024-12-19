use std::fmt::Debug;

// Define traits for validation behavior
trait Validator {
    fn is_valid(&self) -> bool;
    fn validate_and_print(&self);
}

trait Transformable {
    fn transform(&self) -> String;
}

// Generic structure with multiple trait bounds
struct DataValidator<T, U>
where
    T: Debug + Clone,
    U: Debug + Clone,
{
    primary_data: T,
    secondary_data: U,
    validation_enabled: bool,
}

// Implementation with generic constraints
impl<T, U> DataValidator<T, U>
where
    T: Debug + Clone + ToString,
    U: Debug + Clone + ToString,
{
    fn new(primary: T, secondary: U) -> Self {
        DataValidator {
            primary_data: primary,
            secondary_data: secondary,
            validation_enabled: true,
        }
    }

    fn disable_validation(&mut self) {
        self.validation_enabled = false;
    }
}

// Implement Validator trait for specific type combinations
impl<T, U> Validator for DataValidator<T, U>
where
    T: Debug + Clone + ToString,
    U: Debug + Clone + ToString,
{
    fn is_valid(&self) -> bool {
        if !self.validation_enabled {
            return true;
        }
        !self.primary_data.to_string().is_empty() && !self.secondary_data.to_string().is_empty()
    }

    fn validate_and_print(&self) {
        println!(
            "Validation status: {}",
            if self.is_valid() { "Valid" } else { "Invalid" }
        );
        println!("Primary data: {:?}", self.primary_data);
        println!("Secondary data: {:?}", self.secondary_data);
    }
}

// Additional trait implementation
impl<T, U> Transformable for DataValidator<T, U>
where
    T: Debug + Clone + ToString,
    U: Debug + Clone + ToString,
{
    fn transform(&self) -> String {
        format!(
            "Combined data: {} & {}",
            self.primary_data.to_string(),
            self.secondary_data.to_string()
        )
    }
}

fn main() {
    // Create a validator with different types
    let validator = DataValidator::new(String::from("Primary"), 42);

    // Test validation
    validator.validate_and_print();
    println!("Transformed: {}", validator.transform());

    // Create another validator with same types
    let mut same_type_validator = DataValidator::new(String::from("First"), String::from("Second"));

    // Test with validation disabled
    same_type_validator.disable_validation();
    same_type_validator.validate_and_print();
}
