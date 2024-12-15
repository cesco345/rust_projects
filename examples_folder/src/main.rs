use rand::Rng; // Import the Rng trait for random number generation
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng; // Import the ChaCha8 random number generator // Import the SeedableRng trait to seed the RNG

fn main() {
    // Create a new ChaCha8Rng random number generator seeded with 42
    let mut rng = ChaCha8Rng::seed_from_u64(42);

    // Generate a random number between 1 and 100
    let random_number: u32 = rng.gen_range(1..101);

    println!("Random number: {}", random_number);
}
