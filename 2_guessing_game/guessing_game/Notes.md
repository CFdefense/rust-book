# Chapter 2: Guessing Game

## Rust Guessing Game

### We can create a new empty String with String::new()

```rs
let mut guess: String = String::new();
```
Here we initialize a mutable object of type String using String::new()

### Result enum
- Some functions return a Result enumeration which must be handled to avoid errors
    - If this Result returns an Err, Expect() will crash the program and display message
    - If this Result is Ok then it will be used

**Example**:
```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```
Here parse() will return a result of if the parsed guess was valid or not. We then match it accordingly, saving as a number or erroring

### F-String notation
```rs
println!("You guessed: {}", guess)
```
or
```rs
println!("You guessed: {guess}")
```
Rust allows for easy use of fstrings to display information

### Substr/Range notation
```rs
let secret_number = rand::thread_rng().gen_range(1..=100)
```
or
```rs
let secret_number = rand::thread_rng().gen_range(1..101)
```
We can Specify Ranges by using notation such as 1..101 or 1..=100, as shown here where we give a range to a random number generator.

### Match statments

```rs
match guess.cmp(&secret_number) {
    Ordering::Less => print!("Too Small"),
    Ordering::Greater => print!("Too Large"),
    Ordering::Equal => {
        print!("You Win!");
        break
    },
}
```
In this example cmp() allows us to compare using the Ordering trait without boilerplate if and elses.

## Cargo Utility

### Cargo.toml
```rs
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```
We can use 'crates' or packages by adding to 'Cargo.toml', rand = "0.8.5" is one such crate.

### Additional Info
- Cargo.lock is how builds are replicatable. It holds our versions when we first build so that it can be rebuilt.
- Cargo update: ignores Cargo.lock and finds newest package that fits the specifications of Cargo.toml
- Cargo doc --open: Creates documentation on how to use all installed dependencies