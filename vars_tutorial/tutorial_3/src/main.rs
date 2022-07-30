fn main() {
    let x = 4; // implicit typing (int) [immutable]
    let y: u32 = 4; // explicit assignment
    println!("x is : {}", x); // formatted string
    println!("y is : {}", y);

    let mut z : u32 = 5; // mutable var
    println!("z is : {}", z);
    z = 10;
    println!("z is now : {}", z);

    let x = 5; // redefining a declared variable, i.e. changing its value without making it mut
    println!("x is now : {}", x);  

    let x = x +1; // redefining while incrementing
    println!("x is now : {}", x);  

    // name shadowing (scope) example
    {
        let x = 2; 
        println!("x in inner scope is : {}", x);
    }
    println!("x in outer scope is : {}", x);

    // scoping again
    {
        let x = x -2; // using the outer scope x to init the inner scope x
        println!("x in inner scope 2 is : {}", x);
    }
    println!("x in outer scope is : {}", x);

    let x = "heyo!"; // changing types works when redefining vars
    println!("x is now : {}", x);

    // constants ; similar to immutable vars , but different declaration 
    // but cannot be re-assigned or re-defined 
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("Seconds in a minute [constant] : {}", SECONDS_IN_MINUTE);
}
