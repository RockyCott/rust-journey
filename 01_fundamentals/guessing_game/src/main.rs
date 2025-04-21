use rand::Rng;
use std::cmp::Ordering;
use std::io; // std is the standard library, and io is the input/output module

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let random_number: u32 = rand::rng().random_range(1..=100);

    while true {
        println!("Guess a number between 1 and 100:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number.");
                break;
            }
        }
    }
}
