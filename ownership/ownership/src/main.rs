fn main() {
    let str1 = String::from("alo");
    let str2 = str1;

    // borrow of move value
    // println!("{str1}");

    clone_example();

    let str3 = String::from("alo");
    takes_and_back_ownership(str3);

    let str3 = String::from("alo"); // new cannot use old, fn takes ownership of old
    reference_example(str3);
    mut_reference();
    mut_reference2();
}

fn mut_reference() {
    let mut word = String::from("hello");
    let word2 = &mut word;
    // let word3 = &mut word;

    println!("{}", word2);
    // println!("{}", word3);
}

fn mut_reference2() {
    let mut word = String::from("hello");
    let word2 = &mut word;
    println!("{}", word2);
    // cannot use twice mutalbe reference
    let word3 = &mut word;
    println!("{}", word3);
}

fn clone_example() {
    let str1 = String::from("hallo");
    let str2 = str1.clone();

    println!("{} {}", str1, str2);
}

fn takes_and_back_ownership(word: String) -> String {
    word
}

fn reference_example(word: String) -> usize {
    word.len()
} // do not free memory
