fn main() {
    println!("Control flow tutorial, ENGAGE!");

    // Standard operators
    // < , > , <=, >=, !=, ==

    let cond: bool = 2 < 3; // compared literals must be of the same type
    println!("bool cond is : {}", cond);

    let x: f32 = 1.1;
    let y: u8 = 2;
    let cond2: bool = y as f32 > x; // notice the use of casting to allow this to work
    println!("f32 x is : {}", x);
    println!("u8 y is : {}", y);
    println!("bool cond2 (y > x) is : {}", cond2);

    // Logical operators
    // && , ||, !
    let cond3: bool = true && cond;
    println!("bool cond is : {}", cond);
    println!("bool cond3 (true && cond) is : {}", cond3);

    let cond3: bool = true || cond;
    println!("bool cond is : {}", cond);
    println!("bool cond3 (true || cond) is : {}", cond3);

    let cond3: bool = !(true || cond);
    println!("bool cond is : {}", cond);
    println!("bool cond3 (!(true || cond) is : {}", cond3);

    // Order of prescedence
    // ! then && then ||, parentheticals not included

    // Control flow
    let int_check: u8 = 1;
    let int_test: u8 = 1;

    if int_test == int_check {
        println!("u8 int_test({}) == {} is true", int_test, int_check);
    } else if int == 0 {
        println!("u8 int_test is 0");
    } else {
        println!("u8 int_test({}) == {} is false", int_test, int_check);
    }
}
