use std::{error, result};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() {
    println!("Hello, world!");
}
