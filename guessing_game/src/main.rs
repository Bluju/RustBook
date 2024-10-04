use std::io; // Input output library from standard library
use rand::Rng; // Random library for generating random numbers (Rng is needed in scope for functions)
use std::cmp::Ordering; // Comparison library from standard library

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Guess is a mutable string variable bound to an empty instance of a String

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Expect will crash the program with unexpected input

        // Convert guess into a 32 bit unsigned integer
        let guess: u32 = match guess.trim().parse() {
            Ok(number)=> number,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Use pattern matching to determine output
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
