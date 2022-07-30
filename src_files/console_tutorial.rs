// Imports
// format: crate::module
use std::io;
use std::io::Write;

fn main() {
    println!("Console input tutorial, ENGAGE!");
    
    let mut input = String::new();

    print!("Enter data >>: ");    
    let _ = io::stdout().flush(); // flush buffer so stdin doesn't get confused
    
    // Creating mutable reference to the input variable
    // expect = catching any errors
    io::stdin().read_line(&mut input).expect("Error reading from STDIN");
    println!("You entered: {}", input);
}
