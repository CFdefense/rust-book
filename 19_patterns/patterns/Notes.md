## Chapter 18 - OPatterns and Matching: All the Places Patterns Can Be Used

### Overview

`Patterns` pop up in a number of places in Rust, and you’ve been using them a lot without realizing it!

Lets look at all the places we can use them!

### match Arms

Formally, match expressions are defined as such:

1. the keyword match.
2. a value to match on.
3. one or more match arms that consist of a pattern.

Syntax Example:

```rs
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

Heres a match example that matches on an `Option<i32>` value in the variable `x`:

```rs
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

The patterns in this `match` expression are the `None` and `Some(i)` to the left of each arrow.

One **requirement** for `match` expressions is that they need to be **exhaustive** in the sense that *all possibilities* for the value in the `match` expression must be accounted for.

The particular pattern `_` (catch all) will `match` anything, but it never binds to a variable, so it’s often used in the last `match` arm.

### let Statements

Everytime you have used a `let` statement you have actually using `patterns` without realizing it.

For example:

```rs
let x = 5;
```

The above let statement looks like this formally:

```rs
let PATTERN = EXPRESSION;
```

In the above example `x` serves as the `PATTERN` and everything after the equals is the `EXPRESSION`.

Because the name `x` is the *whole pattern*, this pattern effectively means “*bind everything to the variable x, whatever the value is.*”

To see the pattern-matching aspect of `let` more clearly, lets look at a `let` expression that *deconstructs* a `tuple`:

```rs
let (x, y, z) = (1, 2, 3);
```

Here, we match a `tuple` against a `pattern`.

Rust compares the value `(1, 2, 3)` to the pattern `(x, y, z)` and sees that the value matches the pattern.

That is, it sees that the number of elements is the *same* in both sides.

If the number of elements in the pattern **doesn’t match** the number of elements in the `tuple`, the overall type won’t match and we’ll get a **compiler error**.

```rs
let (x, y) = (1, 2, 3);
```

Will then result in the error:

```sh
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0308]: mismatched types
 --> src/main.rs:2:9
  |
2 |     let (x, y) = (1, 2, 3);
  |         ^^^^^^   --------- this expression has type `({integer}, {integer}, {integer})`
  |         |
  |         expected a tuple with 3 elements, found one with 2 elements
  |
  = note: expected tuple `({integer}, {integer}, {integer})`
             found tuple `(_, _)`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

To fix the error, we could ignore one or more of the values in the tuple using `_` or `..`.

### Conditional if let Expressions

`if let` expressions are mainly used as a *shorter way* to write the equivalent of a `match` that only matches **one case**.

**Optionally**, `if let` can have a corresponding `else` containing code to run if the pattern in the `if let` doesn’t match.

It’s also possible to mix and match `if let`, `else if`, and `else if let` expressions.

Doing so gives us *more flexibility* than a `match` expression in which we can express only one value to compare with the patterns.

Also, Rust *doesn’t require* that the conditions in a series of `if let`, `else if`, and `else if let` arms relate to each other.

The following example determines what color to make your background based on a series of checks for several conditions:

```rs
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

The logic is flexible and allows us to ensure the following conditions.

1. If the user specifies a favorite color, that color is used as the background.
2. If no favorite color is specified and today is Tuesday, the background color is green.
3. If the user specifies their age as a string and we can parse it as a number successfully, the color is either purple or orange depending on the value of the number.
4. If none of these conditions apply, the background color is blue.

You can see that if let can also **introduce new variables** that *shadow* existing variables in the same way that `match arms` can:

The line `if let Ok(age) = age` introduces a new `age` variable that contains the value inside the `Ok` variant, *shadowing* the existing `age` variable.

The downside of using `if let` expressions is that the compiler **doesn’t check** for exhaustiveness.

If we **omitted** the last `else` block and therefore missed handling some cases, the compiler *would not* alert us to the possible logic bug.

### while let Conditional Loops

Similar in construction to `if let`, the `while let` conditional loop allows a `while loop` to run for as long as a pattern **continues to match**.

```rs
let (tx, rx) = std::sync::mpsc::channel();
std::thread::spawn(move || {
    for val in [1, 2, 3] {
        tx.send(val).unwrap();
    }
});

while let Ok(value) = rx.recv() {
    println!("{value}");
}
```

This example prints `1`, `2`, and then `3`.

The `recv` method takes the first message out of the `receiver` side of the channel and returns an `Ok(value)`.

We can use `while let`, because the `recv` method returns an `Ok` *each time a message arrives*, as long as the sender exists, and then produces an `Err` once the sender side disconnects.

### for Loops

In a `for` loop, the value that directly follows the keyword `for` is a pattern.

For example, in `for x in y`, the `x` is the pattern.

Lets see how we can use a `for loop` to destructure, or break apart, a `tuple` as part of the `for loop`:

```rs
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{value} is at index {index}");
}
```

We adapt an `iterator` using the `enumerate` method so that it produces a value and the index for that value, placed into a `tuple`.

The first value produced is the tuple `(0, 'a')`. 

When this value is matched to the pattern `(index, value)`, `index` will be `0` and `value` will be `'a'`, printing the first line of the output.

### Function Parameters

**Function parameters** can also be patterns.

Note the following example:

```rs
fn foo(x: i32) {
    // code goes here
}
```

The `x` part is a **pattern**!

As we did with `let`, we could `match` a `tuple` in a function’s arguments to the **pattern**. 

```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

This code prints `Current location: (3, 5)`.

The values `&(3, 5)` match the pattern `&(x, y)`, so `x` is the value `3` and `y` is the value `5`.