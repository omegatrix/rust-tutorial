// Generics
// pub trait Iterator<T> {
//     fn next(&mut self) -> Option<T>;
// }

// Associated types
// When the trait can only be implemented once
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(45)
    }
}

fn main() {
    println!("Hello, world!");
}
