use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guessing Game Part 3, With loop");
    let cpu_guess = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Please enter Any number: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //println!("CPU Guess {}", cpu_guess);
        println!("Your Guess {}", guess);

        match guess.cmp(&cpu_guess) {
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
            Ordering::Greater => println!("Too Big, loss!"),
            Ordering::Less => println!("Too Small, loss!"),
        }
    }
}
