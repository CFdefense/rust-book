## Chapter 3 – Control Flow

### If Expressions
- **Conditions must be `bool`**: Rust won’t automatically convert integers or other types to booleans.
- **`if` is an expression**: you can assign from `if`/`else` branches as long as all branches return the same type.

```rs
let number = 7;
let description = if number % 2 == 0 { "even" } else { "odd" };
println!("{} is {}", number, description);
```

### Looping
- **`loop`**: repeats forever until you `break`.
- **`while`**: runs while the condition is `true`.
- **`for`**: the idiomatic way to iterate over collections and ranges.

```rs
// loop with break value
let mut count = 0;
let result = loop {
    count += 1;
    if count == 3 {
        break count * 2;
    }
};

// while loop
let mut n = 3;
while n != 0 {
    println!("{n}!");
    n -= 1;
}

// for over a range
for i in 1..=3 {
    println!("i = {i}");
}
```

### Extra Notes (from chapter summary)
- Rust will not automatically attempt to convert non-boolean types to booleans for comparisons.
- You can use boolean logic when assigning variables.
- You can loop using `loop`, `while`, or `for`.
- If you have multiple layers of loops, `break` and `continue` apply to the innermost loop unless specified otherwise.

