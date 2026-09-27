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
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}
