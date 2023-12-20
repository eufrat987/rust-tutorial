use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess number");
    println!("input number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
