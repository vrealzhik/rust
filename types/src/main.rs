fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    // i8 u8
    // i16 u16
    // i32 u32
    // i64 u64
    // i128 u128
    // isize usize

    let a: f64 = 2.0;

    let b: f32 = 3.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5; // f32

    let t = true;

    let f: bool = false; 

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
}
