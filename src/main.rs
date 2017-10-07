extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello to the guessing game!");


    let std_in = io::stdin();

    let number_to_guess = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please enter a number: ");
        let mut guess = String::new();
        std_in.read_line(&mut guess).expect("Failed to read line.");
        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
