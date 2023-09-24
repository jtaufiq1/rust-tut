fn main() {
    // Basic mathematical operations
    // Default integer type is i32 (unsigned integer)
    
    // Addition
    let sum = 5 + 10;
    println!("Addition");
    println!("5 + 10 = {sum}");

    // Subtraction
    let diff = 95.5 - 4.3;
    println!("Subtraction");
    println!("95.5 - 4.3 = {diff}");

    // Multiplication
    let product = 4 * 30;
    println!("Multiplication");
    println!("4 * 30 = {product}");

    // Division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("Division");
    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated} # Truncated answer");

    // Remainder
    println!("Remainder");
    let remainder = 43 / 5;
    println!("43 / 5 = {remainder}");
}
