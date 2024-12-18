// Define temperature ranges with associated values
#[derive(Debug)]
enum Temperature {
    Cold(i32), // For temperatures typically below 45°F
    Mild(i32), // For moderate temperatures
    Hot(i32),  // For temperatures typically above 75°F
}

// Define different weather conditions
#[derive(Debug)]
enum Condition {
    Sunny,
    Cloudy,
    Rainy(String), // Includes type of rain (e.g., "light", "heavy")
    Snowy(i32),    // Includes snowfall amount in inches
}

// Combine temperature and condition into a complete weather report
#[derive(Debug)]
struct WeatherReport {
    temp: Temperature,
    condition: Condition,
}

fn generate_weather_advisory(weather: &WeatherReport) {
    // First match on temperature to generate appropriate clothing advice
    let temp_advice = match &weather.temp {
        // Special case for freezing temperatures
        Temperature::Cold(temp) if *temp < 32 => {
            "Freezing conditions! Wear warm layers and watch for ice."
        }
        // General cold weather advice
        Temperature::Cold(temp) => "Cold weather - wear a warm coat!",
        // Mild weather advice
        Temperature::Mild(temp) => "Mild conditions - light jacket recommended.",
        // Special case for extreme heat
        Temperature::Hot(temp) if *temp > 90 => {
            "Very hot! Stay hydrated and avoid extended sun exposure."
        }
        // General hot weather advice
        Temperature::Hot(_) => "Hot weather - stay cool and bring water.",
    };

    // Then match on conditions to generate appropriate precautions
    let condition_advice = match &weather.condition {
        Condition::Sunny => "Don't forget sunscreen!",
        Condition::Cloudy => "No special precautions needed.",
        // Nested match for different types of rain
        Condition::Rainy(intensity) => match intensity.as_str() {
            "light" => "A light raincoat might be useful.",
            "heavy" => "Bring an umbrella and waterproof gear!",
            _ => "Be prepared for rain.",
        },
        // Special case for heavy snow
        Condition::Snowy(inches) if *inches > 6 => {
            "Heavy snowfall expected. Consider postponing travel."
        }
        // General snow advice
        Condition::Snowy(_) => "Watch for snow and ice.",
    };

    // Print the complete advisory
    println!("Weather Advisory:");
    println!("{}", temp_advice);
    println!("{}", condition_advice);
}

fn main() {
    // Create test cases for different weather scenarios
    let reports = vec![
        WeatherReport {
            temp: Temperature::Cold(28),
            condition: Condition::Snowy(8),
        },
        WeatherReport {
            temp: Temperature::Mild(65),
            condition: Condition::Rainy(String::from("light")),
        },
        WeatherReport {
            temp: Temperature::Hot(95),
            condition: Condition::Sunny,
        },
    ];

    // Process each weather report
    for report in reports {
        println!("\nProcessing weather report: {:?}", report);
        generate_weather_advisory(&report);
        println!("------------------------");
    }
}
