extern crate rand;

use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust programming", 96);
    marks.insert("Web Developer", 94);

    println!("how many {}", marks.len());

    let random_number = rand::thread_rng().gen_range(1, 11);
    println!("Random Number: {}", random_number);
}