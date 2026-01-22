## Chapter 6 – `match` Control Flow

### Matching on Enums
- `match` lets you compare a value against patterns and run code for each case.

```rs
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Exhaustiveness
- The compiler enforces that all possible variants are handled (or covered by `_`).

### Extra Notes (from chapter summary)
- `match` allows you to compare a value against a series of patterns and execute code based on which pattern matches.
- Rust will give errors if you don't cover all possible cases in a `match`.


