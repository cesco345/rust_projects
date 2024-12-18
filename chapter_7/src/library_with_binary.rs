// File: src/lib.rs
// This is the library crate root, containing core functionality
// that can be reused by other crates

/// Represents a simple geometric shape
pub enum Shape {
    Circle(f64),         // Radius
    Rectangle(f64, f64), // Width, Height
    Square(f64),         // Side length
}

/// Calculates the area of a given shape
pub fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

/// Calculates the perimeter of a given shape
pub fn calculate_perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Square(side) => 4.0 * side,
    }
}

// Import from our library crate using the package name 'chapter_7'
use chapter_7::geometry::{calculate_area, calculate_perimeter, Shape};

fn main() {
    // Create some example shapes
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Square(3.0),
    ];

    // Process each shape
    for shape in shapes {
        match shape {
            Shape::Circle(r) => println!("Circle with radius {}", r),
            Shape::Rectangle(w, h) => println!("Rectangle with width {} and height {}", w, h),
            Shape::Square(s) => println!("Square with side {}", s),
        }
        println!("Area: {:.2}", calculate_area(&shape));
        println!("Perimeter: {:.2}", calculate_perimeter(&shape));
        println!("-------------------");
    }
}
