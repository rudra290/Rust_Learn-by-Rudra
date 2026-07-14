fn main() {
    let a: i32 = -5;
    let b: i32 = 10;
    let c = sum(a, b);
    println!("Sum of a & b is {}", c);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
