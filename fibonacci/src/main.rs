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
            Ok(_n) => _n,
            Err(_) => continue,
        };
        break n = _n;
    }
    println!("Your Index is {}", n);
    print!("Your Series is: ");
    for i in 1..n {
        print!("{}, ", fib(i));
    }
    print!("{};", fib(n));
}

fn fib(ix: u32) -> u32 {
    if ix == 0 || ix == 1 {
        return ix;
    } else {
        return fib(ix - 1) + fib(ix - 2);
    }
}
