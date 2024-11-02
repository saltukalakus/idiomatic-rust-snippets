extern crate rand;
use rand::Rng;

fn main() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 10
    let random_number: u32 = rng.gen_range(1..=10);

    println!("Random number: {}", random_number);
}