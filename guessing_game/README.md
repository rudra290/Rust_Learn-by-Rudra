# Guessing Game

## A New Bug and a New Lesson

I wrote the following code:

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

Output:

```text
Guess the number
Pick any Number:
7
You guessed 7
```

The cursor appears on the next line after the prompt.

For a better user experience, I changed `println!` to `print!` so the user could type on the same line.

```rust
print!("Pick any Number:");
```

But the output became:

```text
Guess the number
6
Pick any Number:You guessed 6
```

That wasn't what I expected.

---

# Why does this happen?

The answer is **output buffering**.

When we write something using `print!`, Rust doesn't immediately send it to the terminal. Instead, it stores the output in a **buffer**.

A buffer is simply a temporary area in memory where data is collected before being sent to the operating system.

Why?

Sending data to the operating system requires a **system call**, and system calls are relatively expensive compared to ordinary function calls. Instead of making a system call for every character, programs collect data in a buffer and send it all at once.

For example, instead of doing:

```text
print 'P'
system call

print 'i'
system call

print 'c'
system call
```

the runtime collects the characters:

```text
Buffer:
Pick any Number:
```

and sends them together with a single system call.

This greatly improves performance.

---

# Why does `println!` work?

Terminals usually use **line buffering**.

When a newline (`\n`) is written, the runtime automatically flushes the buffer.

So

```rust
println!("Pick any Number:");
```

is effectively

```text
Buffer:
Pick any Number:\n
```

The newline causes the buffer to be flushed immediately, so the prompt appears before the program waits for input.

---

# Why doesn't `print!` work?

`print!` does **not** add a newline.

So the buffer contains

```text
Pick any Number:
```

but nothing has been sent to the terminal yet.

Then the program executes

```rust
io::stdin().read_line(&mut guess);
```

Rust immediately waits for keyboard input.

Since the prompt is still sitting in the output buffer, the user starts typing before the prompt has been displayed.

Eventually the buffer is flushed, so the terminal shows

```text
6
Pick any Number:
```

which looks confusing.

---

# The Solution

Flush the output manually.

```rust
use std::io::{self, Write};

print!("Pick any Number: ");
io::stdout().flush().unwrap();
```

`flush()` tells Rust:

> "Send everything currently in the output buffer to the terminal **right now**."

Now the prompt appears before waiting for input.

---

# Why doesn't this usually happen in C?

This is the interesting part.

Many C programs seem to work without calling `fflush(stdout)`.

For example:

```c
printf("Enter a number: ");
scanf("%d", &x);
```

On Linux, macOS, and many Unix-like systems, the prompt usually appears correctly.

However, this is **not guaranteed by the C standard**.

Most C standard library implementations (such as **glibc**) automatically flush all **line-buffered output streams** before reading from the terminal with functions like `scanf()`.

In other words, the library does something similar to:

```c
fflush(stdout);
scanf(...);
```

behind the scenes when `stdout` is connected to a terminal.

This is a **library feature**, not a language feature.

Rust's standard library **does not perform this automatic flush**. It keeps input and output independent and expects the programmer to flush explicitly when needed.

---

## Was my understanding correct?

Almost.

> "Kernel buffers the output."

❌ Not exactly.

The **standard library** (Rust's `std::io` or C's `stdio`) buffers the output in user-space memory. Only when the buffer is flushed does it make a **system call** (such as `write`) to ask the kernel to send the data to the terminal.

The flow is more like:

```text
Your Program
      │
      ▼
stdout Buffer (Rust/C standard library)
      │
      ▼
write() system call
      │
      ▼
Kernel
      │
      ▼
Terminal
```

So your overall intuition is correct: buffering reduces the number of expensive system calls. The main refinement is that **the buffering happens in the runtime library**, while the **kernel only receives data after a flush or when the buffer is full**.
