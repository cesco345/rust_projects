fn main() {
    // Array of gifts for each day of the song.
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    // Array of day names (first, second, third, etc.).
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Loop through all 12 days.
    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me:",
            days[day] // Print the current day.
        );

        // Loop backward through the gifts for the current day.
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                print!("and "); // Add "and" before the last gift starting from day 2.
            }
            println!("{}", gifts[gift]); // Print the gift for the current iteration.
        }

        println!(); // Print a blank line between days.
    }
}
