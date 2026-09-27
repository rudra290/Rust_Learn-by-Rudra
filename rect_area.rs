struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 12,
        height: 14,
    };
    println!("Area is {}", rect1.area());
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
}
