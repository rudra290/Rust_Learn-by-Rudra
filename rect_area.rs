struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 14,
    };
    println!("Area is {}", area(rect1));
}

fn area(rect: Rectangle) -> u32 {
    return rect.width * rect.height;
}
