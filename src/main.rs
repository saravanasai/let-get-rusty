use std::io;
use rand::Rng;
use colored::*;
fn main() {
    let age = rand::thread_rng().gen_range(1, 127);

    println!("Enter your name");

    let mut name = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read the value");

    let age: String = age.to_string();
    println!("Hello ! {} your predicted age is {}", name.green(),age.red());
}
