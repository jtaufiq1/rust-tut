use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Game: Guess a number
    println!("Guess a number");
    
    // Generate a secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop until correct guess is made.
    loop {
        println!("Input your guess:");

        // Variable to hold input of string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow guess, convert to a number and handle error
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // TODO: Remove line below
        println!("You guessed: {guess}");

        // Compare input to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
