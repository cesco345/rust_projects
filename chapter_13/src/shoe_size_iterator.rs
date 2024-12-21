use your_crate_name::shoes_in_size; // Replace 'your_crate_name' with actual crate name
use your_crate_name::Shoe; // Import Shoe struct as well

fn main() {
    // Create a vector of shoes
    let shoes = vec![
        Shoe {
            size: 8,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 9,
            style: String::from("sandal"),
        },
        Shoe {
            size: 8,
            style: String::from("boot"),
        },
        Shoe {
            size: 10,
            style: String::from("loafer"),
        },
    ];

    // Filter shoes for size 8
    let size_8_shoes = shoes_in_size(shoes, 8);

    // Print the filtered results
    println!("Shoes in size 8:");
    for shoe in size_8_shoes {
        println!("- {} (size {})", shoe.style, shoe.size);
    }
}
