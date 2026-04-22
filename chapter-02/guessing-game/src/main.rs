use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    /* Adding "mut" when defining the variable will make it mutable. Variables are 
    immutable by default in Rust, so you need to explicitly opt in to mutability if you 
    want to change the value of a variable after it’s been initialized.
     */
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut gues)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

}