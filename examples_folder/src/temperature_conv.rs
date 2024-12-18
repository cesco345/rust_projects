// Define an enum to represent different temperature scales
// Adding Clone and Copy traits to allow the enum to be copied instead of moved
#[derive(Debug, Clone, Copy)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

// Function to convert between temperature scales
// Now takes a reference to Temperature to avoid taking ownership
fn convert_temperature(temp: &Temperature) -> (f64, f64, f64) {
    match temp {
        // Match on reference patterns
        Temperature::Celsius(c) => {
            let f = (c * 9.0 / 5.0) + 32.0;
            let k = c + 273.15;
            (*c, f, k)
        }
        Temperature::Fahrenheit(f) => {
            let c = (f - 32.0) * 5.0 / 9.0;
            let k = (f - 32.0) * 5.0 / 9.0 + 273.15;
            (c, *f, k)
        }
        Temperature::Kelvin(k) => {
            let c = k - 273.15;
            let f = (k - 273.15) * 9.0 / 5.0 + 32.0;
            (c, f, *k)
        }
    }
}

fn main() {
    // Create a vector of test temperatures
    let temperatures = vec![
        Temperature::Celsius(25.0),
        Temperature::Fahrenheit(98.6),
        Temperature::Kelvin(310.15),
    ];

    // Iterate over references to the temperatures
    for temp in &temperatures {
        // Pass a reference to convert_temperature
        let (celsius, fahrenheit, kelvin) = convert_temperature(temp);
        println!("\nConverted {:?}:", temp);
        println!("Celsius: {:.2}°C", celsius);
        println!("Fahrenheit: {:.2}°F", fahrenheit);
        println!("Kelvin: {:.2}K", kelvin);
    }
}
