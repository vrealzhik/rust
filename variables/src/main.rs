fn main() {
    let x = 5;

    let x = x + 1;

    let mut y = 10;

    y = y + 10;

    println!("{}", y);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
