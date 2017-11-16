extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    // println!("The secret number is: {}", secret_number);

    println!("Input your Guess");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
    	Ordering::Less    => println!("Too small"),
    	Ordering::Greater => println!("Too big"),
    	Ordering::Equal   => println!("You win"),
    }
}
