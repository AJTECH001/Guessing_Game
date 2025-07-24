use std::io; // Importing the io module from the standard library

use rand::Rng; // Importing the Rng trait from the rand crate

use std::cmp::Ordering; // Importing the Ordering enum for comparison

use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!(" {} ", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "Yow win!".green());
                break;
            }
        }
    }
}
