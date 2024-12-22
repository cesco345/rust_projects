use add_one;
use art::mix;
use art::PrimaryColor;

fn main() {
    // Demonstrate add_one functionality
    let num = 5;
    println!("{} plus one is {}!", num, add_one::add_one(num));

    // Demonstrate art crate functionality
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
    println!("Red and yellow make a secondary color!");
}
