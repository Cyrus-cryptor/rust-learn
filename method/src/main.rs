fn main() {
    let rect = Rectangle {
        width: 70,
        heigh: 90,
    };

    let other = Rectangle {
        width: 12,
        heigh: 12,
    };
    println!("can holds : {}", rect.can_hold(&other));

    let square = Rectangle::square(30);
    println!("area for square is {}", square.area());

    let width = rect.width();
    let width = rect.width;
    println!("The area is {}", rect.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.heigh * self.width
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigh > other.heigh
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigh: size,
        }
    }
}
