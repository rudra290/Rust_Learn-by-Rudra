use std::io::{self, Write};

fn main() {
    let arr = [1, 4, 6, 2, 23];

    print!("Enter a index between 1 to 5: ");
    io::stdout().flush().unwrap();
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Can't read that line");

    let index: usize = index.trim().parse().expect("Not a Number");

    println!("Value at {} is {}", index, arr[index]);
}
