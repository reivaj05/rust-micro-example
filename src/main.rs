use std::io;

fn main() {
    println!("Guess");
    println!("Input your Guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");

    println!("You guessed: {}", guess);
}
