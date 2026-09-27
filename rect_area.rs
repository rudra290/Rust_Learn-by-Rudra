fn main() {
    let dimention = (12, 14);
    println!("Area is {}", area(dimention));
}

fn area(dimention: (u32, u32)) -> u32 {
    return dimention.0 * dimention.1;
}
