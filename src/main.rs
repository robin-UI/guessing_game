use rand::Rng;
use std::{io, u32};

use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let screat_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("our guessed number is {screat_number}.");

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fail to user input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                continue;
            },
        }; //Shadowing

        match guess.cmp(&screat_number) {
            Ordering::Equal => println!("You Won"),
            Ordering::Greater => println!("To Big"),
            Ordering::Less => println!("To small"),
        }
    }
}
