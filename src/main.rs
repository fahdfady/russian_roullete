use colour::*;
use rand::Rng;
use std::io;

fn random_bullet() -> u8 {
    let bullet: u8 = rand::thread_rng().gen_range(1..=6);
    println!("bullet is at {bullet}");
    bullet
}

fn main() {
    println!("You're Playing Russian Roulette.");
    println!("A stick is pointed to your head and it'll be shot a number of times and see where the bullet is");
    loop {
        println!("select a number from 1 - 6");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Filed to read Line");

        if guess.trim().to_lowercase() == "quit"
            || guess.trim().to_lowercase() == "leave"
            || guess.trim().to_lowercase() == "exit"
        {
            break;
        }

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match guess {
            1..=6 => {
                println!("your guess is: {guess}");
                if guess == random_bullet() {
                    red_ln!("DEAD");
                    break;
                } else {
                    green_ln!("You survived");
                }
            }

            _ => {
                e_yellow_ln!("The gun has 6 bullets, fool!")
            }
        }
    }
}
