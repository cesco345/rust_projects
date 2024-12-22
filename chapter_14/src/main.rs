use add_one;
use add_two;
use art::{mix, PrimaryColor, SecondaryColor};
use multiply_utils;

fn main() {
    // Demonstrate add_one functionality
    let num = 5;
    println!("{} plus one is {}!", num, add_one::add_one(num));

    // Demonstrate add_two functionality
    let result = add_two::add_two(num);
    println!("{} plus two is {}!", num, result);

    // Demonstrate multiply_utils functionality
    let doubled = multiply_utils::multiply_by_two(num);
    println!("{} multiplied by two is {}!", num, doubled);

    // Demonstrate art crate functionality
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);

    println!("\nColor mixing demonstration:");
    match orange {
        SecondaryColor::Orange => println!("Successfully mixed red and yellow to make orange!"),
        _ => println!("Something went wrong in the color mixing"),
    }
}
