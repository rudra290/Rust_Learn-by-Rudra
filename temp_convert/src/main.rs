use std::io::{self, Write};

fn read(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn main() {
    let celcius = read("Enter the temprature in Celcius: ");
    // println!("{}", celcius);
    let celcius = match celcius.trim().parse::<f32>() {
        Ok(c) => c,
        Err(_) => 0.0,
    };

    let fahrenheit = (celcius * 9.0 / 5.0 + 32.0) as i32;
    println!("The temprature in Fahrenheit is: {}", fahrenheit);
}
