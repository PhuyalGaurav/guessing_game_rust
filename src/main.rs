use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..2);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to readline");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Number is needed for the guess");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small !"),
        Ordering::Equal => println!("You Won !"),
        Ordering::Greater => println!("Too Big !"),
    }
}
