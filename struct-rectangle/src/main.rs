struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn example2(mut self) -> Self {
        self.height += 2;
        self
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 9,
    };
    let rect3 = Rectangle {
        width: 6,
        height: 4,
    };

    // auto dereference
    println!("Area of rectangle is {}", rect.area());
    println!("Rectangle height is {}", rect.height);
    println!("can hold rect3?: {}", rect.can_hold(&rect3));
    // regular usage of function
    let rect2 = rect.example2();
    println!("Area of rectangle is {}", rect2.area());

    let square = Rectangle::square(4);
}
