use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        guess.clear();
        println!("Please Input Your Guess! : ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Equal => {
                println!("You Won!! ");
                break;
            }
            Ordering::Greater => println!("Too Big !"),
        }
    }
}
