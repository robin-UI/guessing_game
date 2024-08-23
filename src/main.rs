use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let screat_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("our guessed number is {screat_number}.");
    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("fail to user input");

    println!("Your guess number is {guess}")
}