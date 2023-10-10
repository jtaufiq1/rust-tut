use std::env;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check number of arguments
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }
    // Parse the command
    let command = &args[1];

    // Match the command and execute corresponding action
    match command.as_str() {
        "greet" => {
            if args.len() < 3 {
                println!("Please provide a name.");
            } else {
                let name = &args[2];
                greet(name);
            }
        }
        "calculate" => {
            let num1 = &args[2].trim().parse(); //::<f64>();
            let num2 = &args[3].trim().parse(); //::<f64>();

            match(num1, num2) {
                (Ok(n1), Ok(n2)) => calculate(num1, num2),
                _ => println!("Invalid input. Please provide valid numbers."),
            }
        }
        _ => {
            println!("Unknown command: {command}");
            println!("available commands: greet, calculate");
        }
    }
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn calculate(num1: f64, num2: f64) {
    let sum = num1 + num2;
    let difference = num1 - num2;
    let product = num1 * num2;
    let quotient = num1 / num2;
    let remainder = num1 % num2;

    println!("Sum: {}", sum);
    println!("Difference: {}",difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}
