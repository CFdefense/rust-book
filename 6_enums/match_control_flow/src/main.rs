fn main() {
    println!("Hello, world!");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    // Control Flow Using Match
    match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
    }

    // Or can use this syntax to handle other
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

}

// We can have enums within other enums for complex data structures
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState) => 25,
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

// We can also use match to destructure enums
fn value_in_cents_two(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Function that uses matching on an optional value
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}