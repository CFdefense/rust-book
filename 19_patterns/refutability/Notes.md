## Chapter 19 - Patterns and Matching: Refutability: Whether a Pattern Might Fail to Match

### Overview

Patterns come in two forms: `refutable` and `irrefutable`.

Patterns that will match for *any* possible value passed are `irrefutable`.

An example would be `x` in the statement `let x = 5;` because `x` matches anything and therefore *cannot fail* to match. 

Patterns that can *fail to match* for some possible value are `refutable`.

An example would be `Some(x)` in the expression `if let Some(x) = a_value`.

If the value in the `a_value` variable is `None` rather than `Some`, the `Some(x)` pattern *will not match*.

*Function parameters*, *let statements*, and *for loops* can **only** accept `irrefutable` patterns because the program **cannot do anything meaningful** when values don’t match.

The `if let` and `while let` expressions and the `let...else` statement accept `refutable` **and** `irrefutable` patterns.

The compiler will, however, warn against `irrefutable` patterns because, by definition, they’re intended to handle possible failure: 

The functionality of a `conditional` **is in its ability** to perform differently depending on success or failure.

In *general*, you shouldn’t have to worry about the distinction between `refutable` and `irrefutable` patterns.

However, you do need to be *familiar* with the concept of `refutability` so that you can respond when you see it in an error message.

In those cases, you’ll need to change either the **pattern** or the **construct** you’re using the pattern with, *depending* on the intended behavior of the code.

Let’s look at an example of what happens when we try to use a `refutable` pattern where Rust **requires** an `irrefutable` pattern and vice versa.

#### Rufutable used but irrefutable expected

Lets use a `let statement`, but for the pattern, we’ve specified `Some(x)`, a `refutable` pattern. 

This code does not compile!

```rs
let Some(x) = some_option_value;
```

We will see the following error:

```sh
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error[E0005]: refutable pattern in local binding
 --> src/main.rs:3:9
  |
3 |     let Some(x) = some_option_value;
  |         ^^^^^^^ pattern `None` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch19-02-refutability.html
  = note: the matched value is of type `Option<i32>`
help: you might want to use `let else` to handle the variant that isn't matched
  |
3 |     let Some(x) = some_option_value else { todo!() };
  |                                     ++++++++++++++++

For more information about this error, try `rustc --explain E0005`.
error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

If `some_option_value` were a `None` value, it would *fail to match* the pattern `Some(x)`, meaning the pattern is `refutable`.

However, the `let statement` can **only accept** an `irrefutable` pattern because there is nothing valid the code can do with a `None` value.

At **compile time**, Rust will complain that we’ve tried to use a `refutable` pattern where an `irrefutable` pattern is required.

A simple fix here is to use a pattern.

Instead of using `let`, we can use `let...else`. Then, if the pattern *doesn’t match*, the code in the curly brackets will handle the value.

```rs
let Some(x) = some_option_value else {
    return;
};
```

We’ve given the code an out! This code is **perfectly valid**, although it means we cannot use an `irrefutable` pattern without receiving a warning.

#### Irrefutable used but refutable expected

If we give `let...else` a pattern that will **always match**, such as `x`, we will get a warning.

```rs
let x = 5 else {
    return;
};
```

Rust complains that it *doesn’t make sense* to use `let...else` with an `irrefutable` pattern:

```sh
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
warning: irrefutable `let...else` pattern
 --> src/main.rs:2:5
  |
2 |     let x = 5 else {
  |     ^^^^^^^^^
  |
  = note: this pattern will always match, so the `else` clause is useless
  = help: consider removing the `else` clause
  = note: `#[warn(irrefutable_let_patterns)]` on by default

warning: `patterns` (bin "patterns") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/patterns`
```

For this reason, `match arms` must use `refutable` patterns, **except** for the last arm, which should match *any remaining values* with an `irrefutable` pattern. 

Rust allows us to use an `irrefutable` pattern in a match with only one arm, but this syntax isn’t particularly useful and could be replaced with a simpler `let statement`.