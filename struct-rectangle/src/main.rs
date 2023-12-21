struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 9,
    };

    println!("Area of rectangle is {}", area(&rect));
    println!("Rectangle height is {}", rect.height);
}
