use std::net::IpAddr;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // we code the constructor such that it will panic if given a value outside of 1-100
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // an example of when expect is verbose and useful
    let _home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

    // lets mess around with custom types for validation
        loop {
        // --snip--
        // some users guess
        let guess = "100";

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        // match guess.cmp(&secret_number) {
        //     // --snip--
        // }
    }
}