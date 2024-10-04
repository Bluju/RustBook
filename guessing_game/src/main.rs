use std::io; // input output library from standard library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // Guess is a mutable string variable bound to an empty instance of a String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // expect will crash the program with unexpected input

    println!("You guessed: {}", guess);
}
