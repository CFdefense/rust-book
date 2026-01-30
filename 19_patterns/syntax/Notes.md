## Chapter 19 - Patterns and Matching: Pattern Syntax

### Overview

In this section, we gather all the syntax that is valid in patterns and discuss why and when you might want to use each one.

#### Matching Literals

You can match patterns against literals directly. The following code gives some examples:

```rs
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This code prints `one` because the value in `x` is `1`. This syntax is useful when you want your code **to take an action if it gets a particular concrete value**.

#### Matching Named Variables

**Named variables** are `irrefutable` patterns that match *any value*, and we’ve used them many times in this book.

However, there is a **complication** when you use **named variables** in `match`, `if let`, or `while let` expressions.

This is because each of these kinds of expressions starts a *new scope*.

Variables declared as part of a pattern inside these expressions will **shadow** those with the **same name** outside the constructs.

To demonstate this lets declare a variable named `x` with the value `Some(5)` and a variable `y` with the value `10`.

We then create a `match` expression on the value `x`.

```rs
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {y}"),
    _ => println!("Default case, x = {x:?}"),
}

println!("at the end: x = {x:?}, y = {y}");
```

Let’s walk through what happens when the `match` expression runs.

The pattern in the first match arm doesn’t match the defined value of `x`, so the code continues.

The pattern in the second match arm introduces a new variable named `y` that will **match** any value inside a `Some` value. 

Because we’re in a **new scope** inside the match expression, this is a **new** `y` variable, *not* the `y` we declared at the beginning with the value `10`. 

This **new** `y` binding will match **any value** inside a `Some`, which is what we have in `x`.

Therefore, this **new** `y` binds to the inner value of the `Some` in `x`.

That value is `5`, so the expression for that arm **executes** and prints `Matched, y = 5`.

If `x` had been a `None` value instead of `Some(5)`, the patterns in the first two arms *wouldn’t have matched*, so the value would have matched to the underscore.

We didn’t introduce the `x` variable in the pattern of the underscore arm, so the `x` in the expression **is still** the outer `x`.

When the `match` expression is done, its scope ends, and so does the scope of the inner `y`.

The last `println!` produces `at the end: x = Some(5), y = 10`.

To create a `match` expression that compares the values of the **outer** `x` and `y`, *rather than introducing a new variable* that `shadows` the existing `y` variable:

We would need to use a `match guard conditional` instead. 

#### Matching Multiple Patterns