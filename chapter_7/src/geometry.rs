// Module containing geometry-related functionality
// Each item is marked with pub to make it accessible from outside the module

#[derive(Debug)]
pub enum Shape {
    Circle(f64),         // Radius
    Rectangle(f64, f64), // Width, Height
    Square(f64),         // Side length
}

pub fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}

pub fn calculate_perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
        Shape::Rectangle(width, height) => 2.0 * (width + height),
        Shape::Square(side) => 4.0 * side,
    }
}
