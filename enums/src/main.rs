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
    PLN(u8),
    USD,
}

fn main() {
    let optional: Option<Message> = None;
    let ip = IpAddrKind::V4(192, 168, 0, 1);
    let msg1 = Message::Move { x: 23, y: 15 };
    msg1.call();
    println!("Hello, world!");
    exchange(Currency::USD);
    dice_match(9);
    if_let_example(Some(7));
}

fn if_let_example(num: Option<i32>) {
    if let Some(x) = num {
        println!("{}", x);
    }
}

fn dice_match(roll: u8) {
    match roll {
        3 => println!("3"),
        7 => println!("7"),
        _ => println!("other"),
    }
}

fn add_one(number: Option<i32>) -> Option<i32> {
    match number {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn exchange(curr: Currency) -> f64 {
    match curr {
        Currency::PLN(year) => {
            println!("year {}", year);
            1.0
        }
        Currency::USD => 5.3,
    }
}
