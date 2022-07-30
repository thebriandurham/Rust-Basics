use std::io;
use std::io::Write;

fn main() {
    println!("Arithmetic & Type Casting tutorial, ENGAGE!");

    let x: u8 = 254; // 0 - 255
    let y: u8 = 1; // -128 - 127

    let z: u8 = x + y; // ok

    println!("u8 x is : {}", x);
    println!("u8 y is : {}", y);
    println!("u8 z is : {}", z);

    let x: u8 = 255;
    let y: u8 = 10;
    let z: u8 = x / y;

    println!("u8 x is : {}", x);
    println!("u8 y is : {}", y);
    println!("u8 z is : {}", z); // notice it returns an int, since z is u8 and not float    

    let x: f64 = 255.0;
    let y: f64 = 10.0;
    let z: f64 = x / y;

    println!("f64 x is : {}", x);
    println!("f64 y is : {}", y);
    println!("f64 z is : {}", z); // proper float result

    let z: f64 = x % y;
    println!("x % y is : {}", z);

    let _x = 255u8; // you can also declare type here
    let _y = 10_i8; // this is also okay
    let _z = 127_000_i64; // == 127,000, this helps make values more readable

    let x = 127_000 as i64; // this works too , and is also explicit type conversion
    let y = 10_i32;
    let z: i64 = x / (y as i64);
    let z: i64 = x / y as i64; // parenthesis are optional here
    println!("i64 x is : {}", x);
    println!("i32 y is : {}", y);
    println!("i64 z (x/y) is : {}", z);

    let x: i64 = (i32::MAX as i64) + 1; // check out that fun i32 function call
    let y: i32 = 10;
    let z: i64 = x / y as i64;
    let a: i32 = x as i32 / y;
    println!("i64 x is : {}", x);
    println!("i32 y is : {}", y);
    println!("i64 z (x/y) is : {}", z);
    println!("i32 a (x/y) is : {}", a); // this triggers an overflow that doesn't get caught, so be careful

    // Converting string from user input to a number
    print!("Enter an integer: ");
    let mut input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error from STDIN");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("You entered: {}", int_input);
    println!("Your input + 1 is : {}", int_input + 1);
}
