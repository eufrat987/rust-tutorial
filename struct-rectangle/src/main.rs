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
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 9,
    };

    // auto dereference
    println!("Area of rectangle is {}", rect.area());
    println!("Rectangle height is {}", rect.height);
    // regular usage of function
    let rect2 = rect.example2();
    println!("Area of rectangle is {}", rect2.area());
}
