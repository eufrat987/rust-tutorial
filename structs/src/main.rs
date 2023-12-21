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

    let user2 = new_user(String::from("test"), String::from("test@email.test"));

    println!("{}", user1.username);
}

fn new_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
    }
}
