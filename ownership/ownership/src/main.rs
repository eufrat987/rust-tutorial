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
    slice_example();
}

fn first_word(a: &str) -> &str {
    let bytes = a.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &a[..i];
        }
    }

    &a[..]
}

fn slice_example() {
    let mut words = String::from("hello");

    let word = first_word(&words[..]);

    words.clear();

    // cannot do - nice
    // println!("{word}");
}

// cannot create
// fn dangle() -> &String {
//     let word = String::from("hello");
//
//     &word
// }

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

    let word3 = &mut word;
    println!("{}", word3);
}

fn mut_reference3() {
    let mut word = String::from("hello");
    let word3 = &word;
    let word4 = &word;

    // cannot have mut and immutable refences
    // let word2 = &mut word;
    // println!("{} {} {}", word3, word4, word2);
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
