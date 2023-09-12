use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is not much of a secret, it is: {secret_number}");

    loop {
        println!("Please guess a number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number and only a number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("Congratulations! You have bested the random number generator. At least for today...");
                break;
            }
        }
    }
}
