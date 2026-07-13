# Rust_Learn-by-Rudra
This is my Rust Learning Journey

### Day 3
- Rust's all veriables are immutable.
- Rust have constant, same immutable. It's naming is in all capital latter.

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.

The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
