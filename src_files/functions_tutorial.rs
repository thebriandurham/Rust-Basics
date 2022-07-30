fn main() {
    println!("Functions, Expressions, & Statements tutorial, ENGAGE!");

    test_fn_one();
    add_numbers(1,2);
    expression_example();

    let x: i32 = add_numbers_with_ret(2,3);
    println!("i32 x (2+3) is {}", x);

    println!("early_return_fn(2) returns {}", early_return_fn(2));
    println!("early_return_fn(1) returns {}", early_return_fn(1));

    println!("explicit_return_fn() returns {}", explicit_return_fn());
}

// naming convention == snake case
// note: fuctions can be declared in any order
fn test_fn_one() {
    println!("test_fn_one called!");
}

// function with args
fn add_numbers(x:u8, y:u8) {
    println!("add_numbers(args[]) called!");
    println!("{} + {} = {}", x, y, x + y);
}

// rust functions can return an expression, but they cannot return a statement
// in rust:
//      statement = ~ variable declaration , e.g. let x = 20;
//          let y = (let x = 20); is not allowed
//          let x = fn add_numbers().... ; also not allowed
//
//      expression = ~ anything that evaluates to something or returns a value
//          a value is an expression (e.g.: 20, 2 < 3, 2 + 3, etc.)
//          a function call is an expression
//          a macro is an expression

fn expression_example(){
    // this is also an expression
    let number = {
        let x = 3;
        x + 1 // do *not* put a ; here so the value will be returned to number
    };
    println!("number is : {}", number);
}

// Returning values from functions
fn add_numbers_with_ret(x: i32, y:i32) -> i32 {
    // as much code as you want can go here, but the last line must be an expression to be returned

    x + y // notice the lack of ; which lets it return
}

// example of a function with an early return
fn early_return_fn(x:u8) -> bool{
    if x == 1 {
        return false; // it's okay to have the ; here
    }
    true
}

fn explicit_return_fn() -> bool {
    return true; // this works too
}