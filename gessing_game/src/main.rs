use rand::RngExt;
use std::io;
use std::cmp::Ordering;

fn main() {
    // Generate a random number between 1 and 100
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("Debug: The secret number is: {}", secret_number);
    loop {
        println!("Guess the number between 1 and 100!");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Please guess the number!");
}
