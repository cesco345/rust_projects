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
