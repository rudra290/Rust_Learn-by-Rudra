use std::io::{self, Write};

use rand::Rng;

fn main() {
    println!("Guessing Game Part 2");
    print!("Please enter Any number: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let cpu_guess = rand::thread_rng().gen_range(1..=100);
    println!("CPU Guess {}", cpu_guess);
    println!("Your Guess {}", guess);
}
