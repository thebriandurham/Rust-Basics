fn main() {
    println!("Data types tutorial: ENGAGE!");

    // Primitives :: Scalar, Compound
    // Scalar = singular value (int, bool, etc)

    let x: i32 = -2; // signed INT 32 bits
    let y: u32 = 3; // unsigned INT 32 bits

    println!("Signed int 32 (i32) x is : {}", x);
    println!("Unsigned int 32 (u32) y is : {}", y);

    let floating_point_32: f32 = 10.9;
    let floating_point_64: f64 = -12.412;

    println!("Floating point 32 (f32) is : {}", floating_point_32);
    println!("Floating point 64 (f64) is : {}", floating_point_64);

    let bool_test: bool = false;
    let bool_test_2: bool = true;

    println!("Bool bool_test is : {}", bool_test);
    println!("Bool bool_test_2 is : {}", bool_test_2);

    let letter: char = 'a';

    println!("char letter is : {}", letter);

    // Primitives :: compounds
    // Compound = multiple values (array, tuple)
    let tup: (u32, bool, i32, char) = (1, true, -5, 'a');
    let tup_2: (bool, char, u8) = (false, 'b', 2); 
    println!("tuple tup is : {:?}", tup);
    println!("tuple tup_2 is : {:?}", tup_2);
    println!("the first value of tuple tup_2 is {}", tup_2.0);

    let mut tup_3: (bool, bool, bool) = (false, false, false);
    println!("mutable tuple tup_3 is : {:?}", tup_3);
    tup_3.0 = true;
    println!("mutable tuple tup_3 is now : {:?}", tup_3);

    // Arrays
    let arr: [u8; 5] = [1,2,3,4,5];
    println!("array arr is : {:?}", arr);
    let mut arr_2: [bool; 3] = [false,false,false];
    println!("mutable array arr_2 is : {:?}", arr_2);
    arr_2[0] = true;
    println!("mutable array arr_2 is now : {:?}", arr_2);
}
