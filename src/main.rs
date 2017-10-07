extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello to the guessing game!");


    let std_in = io::stdin();

    let number_to_guess = rand::thread_rng().gen_range(1, 101);

    const MAX_TRIES: u32 = 6;
    let mut tries = 0;

    loop {
        if tries == MAX_TRIES {
            println!("Too many tries, you lose :/");
            break;
        }

        println!("Please enter a number: ");
        let mut guess = String::new();
        std_in.read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
        tries += 1;
    }
}
