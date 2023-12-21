struct User {
    active: bool,
    username: String,
    email: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("email@test.pl"),
    };

    user1.username = String::from("username2");

    println!("{}", user1.username);
}
