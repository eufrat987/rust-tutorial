enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip = IpAddrKind::V4(192, 168, 0, 1);
    println!("Hello, world!");
}
