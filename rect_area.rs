fn main() {
    let height = 12;
    let width = 14;
    println!("Area is {}", area(height, width));
}

fn area(width: u32, height: u32) -> u32 {
    return height * width;
}
