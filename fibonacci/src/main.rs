use colored::*;
use std::io::{self, Write};
fn main() {
    println!("Fibonacci Series");
    let n: u32;
    loop {
        print!("Enter The last index till you want to Print Series: ");
        io::stdout().flush().unwrap();
        let mut _n = String::new();

        io::stdin().read_line(&mut _n).expect("Failed to read line");

        let _n: u32 = match _n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("{}", "Invalid input! Please enter a +ve number only.".red());
                continue;
            }
        };
        break n = _n;
    }
    let mut a: usize = 0;
    let mut b: usize = 1;
    println!("Your Index is {}", n);
    print!("Your Series is: ");
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
        print!("{} ", b);
    }
}
