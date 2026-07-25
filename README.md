# Rust_Learn-by-Rudra
This is my Rust Learning Journey

### Day 3
- Rust's all veriables are immutable.
- Rust have constant, same immutable. It's naming is in all capital latter.

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

### Probably After two weeks

I learn that fixed size and small data type becomes copy. I think that's not true. If we can directly store data in Varable then when you allocate it to another, Another have copy of first one's data.
```rust
let a = 5;
let b = a;
```
Now b have copy of a. When we introduce strings, Variable is pointing to that huge string data. One more thing, string have pointer, length and capacity.
```rust
let a = String::from("Hello");
let b = a;
println!("{a}"); // a is not valid here
```
In normal languages, a and b is pointing same data. In language of rust. Both are owner of that data. Which rust is priventing by it's compiler. That's why at last you can see a is not valid at printing.
