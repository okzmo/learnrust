use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Ready to guess a number between 0 and 100 ?");

    let number: u32 = rand::thread_rng().gen_range(0..=100);
    let mut guess = String::new();

    loop {
        println!("Please input your guess:");
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}!", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small! Guess again."),
            Ordering::Greater => println!("Too big! Guess again."),
            Ordering::Equal => {
                println!("Seems like you guessed it!");
                break;
            }
        }
    }
}
