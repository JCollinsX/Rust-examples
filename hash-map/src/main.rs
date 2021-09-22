use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust programming", 96);
    marks.insert("Web Developer", 94);

    println!("how many {}", marks.len());
}