fn main() {
    let str1 = String::from("alo");
    let str2 = str1;

    // borrow of move value
    // println!("{str1}");

    clone_example();
}

fn clone_example() {
    let str1 = String::from("hallo");
    let str2 = str1.clone();

    println!("{} {}", str1, str2);
}
