const SOME_CONST: u32 = 10 * 20;

fn main() {
    let x = 5;
    let mut y = 8;

    println!("{y}");

    y = 7;

    println!("{x}");
    println!("{y}");
    println!("{SOME_CONST}");
}
