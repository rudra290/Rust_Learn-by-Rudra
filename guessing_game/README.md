# guessing Game

### New bug and New learn

I type code like
```rust
use std::io;

fn main() {
    println!("Guess the number");
    println!("Pick any Number:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Didn't enter a valid string");

    println!("You guessed {}", guess);
}
```
Give me output
```
Guess the number
Pick any Number:
7
You guessed 7
```

The input pointer is below **Pick any Number:** Line. So I decided to use `print!` insted. So I can get good user experince. Right ? 
Output:
```
Guess the number
6
Pick any Number:You guessed 6
```
Not as Expected! Why ?

The concept of Buffers.
> Since the print and scan function are related to hardware. We need to request kernal to do so. And kernal is activated by syscall(). Which increase overhead to print every character.
> In programming. We use buffers, whatever to print. Put it in buffer and then call kernal. It will printout all buffer stored item. By default \n used to indicate flush or call kernal. So when new line comes, printout previous one, then fill buffer of next line.
> Printing is work on screen. std::out and Input done by keyboard std::in. Not dependent on each other. 
> And at *print!* It just stored in buffer. Waiting for a reason to print. Meanwhile, scan comes, and want input. So we gives input first and then printing out rest of thing.

### Solution ?
```rust
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
```
This prevent the problem and menually flushed out the things in buffer. But why not in other languages ?? 
Rust gives fredom, In C it's solved by stdio library it self. When scanf comes, first try to check if any buffer is full or not.
