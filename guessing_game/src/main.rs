use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");

    let secret_num = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input the number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error number, plz retry!");
                continue;
            }
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it, GG");
                break;
            }
        }
    }
}
