enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call mesage");
    }
}

enum Currency {
    PLN,
    USD,
}

fn main() {
    let optional: Option<Message> = None;
    let ip = IpAddrKind::V4(192, 168, 0, 1);
    let msg1 = Message::Move { x: 23, y: 15 };
    msg1.call();
    println!("Hello, world!");
    exchange(Currency::USD);
}

fn exchange(curr: Currency) -> f64 {
    match curr {
        Currency::PLN => 1.0,
        Currency::USD => 5.3,
    }
}
