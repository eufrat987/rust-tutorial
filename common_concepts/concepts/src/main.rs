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
    tuple_example();
    arrays();
    expressions();
    let if_exp = ifexpr(3);
    println!("{if_exp}");
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
    let _i1: i8 = 127;
    let _i2: i16 = 32767;
    let _i3: i32 = 2147483647;
    let _i4: i64 = 9_223_372_036_854_775_807;
    let _i5: i128 = 2 ^ 128 - 1;
}

fn tuple_example() {
    let t = (1, false, 2.12);
    let (x, y, z) = t;

    println!("{x}, {y}, {z}");
}

fn arrays() {
    let _aa = [1, 2, 3, 4];
    let _aaa = [3; 5];
}

fn expressions() {
    let x = {
        let xx = 5;
        xx + 3
    };
    println!("{x}");
}

fn ifexpr(y: i32) -> i32 {
    let x = if y > 2 { 5 } else { 6 };
    x
}
