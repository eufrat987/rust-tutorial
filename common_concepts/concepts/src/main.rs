const SOME_CONST: u32 = 10 * 20;

fn main() {
    let x = 5;
    let mut y = 8;

    println!("{y}");

    y = 7;

    println!("{x}");
    println!("{y}");
    println!("{SOME_CONST}");

    shadow();
    data_types();
}

fn shadow() {
    let x = 8;
    {
        let x = 6;
        println!("{x}");
    }

    println!("{x}");
}

fn data_types() {
    let i1: i8 = 127;
    let i2: i16 = 32767;
    let i3: i32 = 2147483647;
    let i4: i64 = 9_223_372_036_854_775_807;
    let i5: i128 = 2 ^ 128 - 1;
}
