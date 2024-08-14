// fn main() {
//     println!("Hello, world!");

//     another_function(5);
//     print_labels(5, 'h');
//     let x = plus_one(5);
//     println!("{x}");
// }

fn another_function(x: u32) {
    println!("Another function. : {x}");
}

fn print_labels(x: i32, y: char) {
    println!("Labels: {x} {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
