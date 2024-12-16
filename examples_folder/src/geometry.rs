use std::f64::consts::PI; // Import PI for circle calculations

// Struct representing a 2D point
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Associated function to create a new point
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Method to calculate distance between two points
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Struct representing a circle
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    // Associated function to create a new circle
    fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    // Method to calculate the area of the circle
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    // Method to check if a point is inside the circle
    fn contains_point(&self, point: &Point) -> bool {
        self.center.distance(point) <= self.radius
    }
}

fn main() {
    // Create two points
    let point1 = Point::new(0.0, 0.0);
    let point2 = Point::new(3.0, 4.0);

    println!("Point 1: {:?}", point1);
    println!("Point 2: {:?}", point2);

    // Calculate distance between points
    let distance = point1.distance(&point2);
    println!("Distance between Point 1 and Point 2: {:.2}", distance);

    // Create a circle with Point1 as center and radius 5.0
    let circle = Circle::new(point1, 5.0);

    println!("Circle: {:?}", circle);
    println!("Area of the circle: {:.2}", circle.area());

    // Check if Point2 is inside the circle
    if circle.contains_point(&point2) {
        println!("Point 2 is inside the circle.");
    } else {
        println!("Point 2 is outside the circle.");
    }
}
