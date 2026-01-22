use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Input Your Guess");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too Small"),
            Ordering::Greater => print!("Too Large"),
            Ordering::Equal => {
                print!("You Win!");
                break
            },
        }
    }
}
