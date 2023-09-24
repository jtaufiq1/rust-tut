const CONST_VALUE: u32 = 10;

fn main() {
    println!("Rust Variables & Mutability");
    println!("Variables are immutable by default");
    println!("Add the *mut keyword to make a variable mutable");

    let x = 5;

    // Shadowing
    let x = x * 1;

    {
        let x = x * 2;
        println!("The valule of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Constant
    println!("CONSTANT VALUE: {CONST_VALUE}");
}
