use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

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

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&cpu_guess) {
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too Big, loss!"),
        Ordering::Less => println!("Too Small, loss!"),
    }
}
