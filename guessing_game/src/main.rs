use std::io::{self, Write};

fn main() {
    println!("Guess the number");

    print!("Pick any Number: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
