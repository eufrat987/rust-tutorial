fn main() {
    let str1 = String::from("alo");
    let str2 = str1;

    // borrow of move value
    // println!("{str1}");

    clone_example();

    let str3 = String::from("alo");
    takes_and_back_ownership(str3);
}

fn clone_example() {
    let str1 = String::from("hallo");
    let str2 = str1.clone();

    println!("{} {}", str1, str2);
}

fn takes_and_back_ownership(word: String) -> String {
    word
}
