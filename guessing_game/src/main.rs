use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess string into a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // Match the guess to secret number. Use arms for cases
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Wrong!!!! Guess Lower."),
            Ordering::Less => println!("Wrong!!!! Guess Higher."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
