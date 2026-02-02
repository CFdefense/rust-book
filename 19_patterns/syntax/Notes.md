## Chapter 19 - Patterns and Matching: Pattern Syntax

### Overview

In this section, we gather all the syntax that is valid in patterns and discuss why and when you might want to use each one.

### Pattern Matching

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

In match expressions, you can match multiple patterns using the `|` syntax.

For example, in the following code, we match the value of `x` against the match arms:

The first of which has an `or` option, meaning if the value of `x` matches **either** of the values in that arm, that arm’s code will run.

```rs
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

This code prints `one or two`.

#### Matching Ranges of Values with ..=

The `..=` syntax allows us to match to an **inclusive** range of values.

```rs
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
```

If `x` is `1`, `2`, `3`, `4`, or `5`, the first arm *will match*.

This syntax is more convenient for multiple match values than using the `|` operator, when we want to match this many values.

You can also use `char`:

```rs
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
```

Rust can tell that `'c'` is within the *first pattern’s range* and prints `early ASCII letter`.

### Destructuring to Break Apart Values

We can also use patterns to **destructure** `structs`, `enums`, and `tuples` to use different parts of these values. 

Let’s walk through each value.

#### Structs

Here we have a `Point` struct with two fields, `x` and `y`, that we can *break apart using a pattern* with a `let statement`:

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

This code creates the variables `a` and `b` that **match** the values of the `x` and `y` fields of the `p` struct.

There is another **short hand** version of this syntax to avoid duplicated code:

```rs
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

This code creates the variables `x` and `y` that match the `x` and `y` fields of the `p` variable.

We can also *destructure* with **literal** values as part of the `struct` pattern rather than creating variables for all the fields.

Doing so allows us to **test** some of the fields for particular values while creating variables to destructure the other fields.

Lets see an Example:

We have a `match` expression that separates `Point` values into three cases: 

1. points that lie directly on the x axis (which is true when y = 0).
2. points that lie directly on the y axis (x = 0).
3. points that lie on neither axis.

```rs
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

The **first arm** will match *any point that lies on the x axis* by specifying that the `y` field matches if its value **matches the literal 0**.

The pattern still creates an `x` variable that we can use in the code for this arm.

Similarly, the **second arm** matches *any point on the y axis* by specifying that the `x` field matches if its value is `0` and creates a variable `y` for the value of the `y` field.

The **third arm** doesn’t specify any literals, so it matches *any other Point* and creates variables for both the `x` and `y` fields.

In this example, the value `p` matches the **second arm** by virtue of `x` containing a `0`, so this code will print `On the y axis at 7`.

Remember that a `match expression` stops checking arms once it has found the **first matching pattern**.

So even though `Point { x: 0, y: 0 }` is on the **x axis** and the **y axis**, this code would only print `On the x axis at 0`.

#### Enums

We've *deconstructed* enums before, lets take a look at how to concretely do it using a `Message` enum.

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }
}
```

This code will print `Change color to red 0, green 160, and blue 255`.

For enum variants without any data, like `Message::Quit`, we can’t destructure the value any further. 

We can only match on the literal `Message::Quit` value, and no variables are in that pattern.

For *struct-like enum variants*, such as `Message::Move`, we can use a pattern similar to the pattern we specify to match structs.

For *tuple-like enum variants*, like `Message::Write` that holds a tuple with one element and `Message::ChangeColor` that holds a tuple with three elements, the pattern is similar to the pattern we specify to match tuples.

#### Nested Structs and Enums

So far, our examples have all been matching structs or enums **one level deep**, but matching can work on **nested items too**! 

For example, we can refactor the code to support *RGB* and *HSV* colors in the `ChangeColor` message:

```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }
}
```

The pattern of the **first arm** in the match expression matches a `Message::ChangeColor` enum variant that contains a `Color::Rgb` variant.

Then, the pattern *binds* to the three inner `i32` values.

The pattern of the **second arm** also matches a `Message::ChangeColor` enum variant, but the inner enum matches `Color::Hsv` instead.

We can specify these complex conditions in **one match expression**, even though **two** enums are involved.

#### Structs and Tuples

We can mix, match, and nest destructuring patterns in even more complex ways.

The following example shows a *complicated destructure* where we nest structs and tuples inside a tuple and destructure all the primitive values out:

```rs
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

This code lets us break complex types into their component parts so that we can use the values we’re interested in separately.

### Ignoring Values in a Pattern

We've seen that it’s sometimes useful to ignore values in a pattern, such as in the last arm of a match.

To get a catch-all that doesn’t actually do anything but does account for all remaining possible values.

There are a few ways to *ignore* entire values. Lets take a look.

#### An Entire Value with _

We’ve used the *underscore* as a wildcard pattern that will match any value but not bind to the value.

This is especially useful as the **last arm** in a match expression, but we can also use it in **any pattern**.


```rs
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);
}
```

This code will **completely ignore** the value `3` passed as the first argument, and will print `This code only uses the y parameter: 4`.

This is useful in someplaces where the compiler enforces including a variable but in the implementation you actually do not need it.

#### Parts of a Value with a Nested _

We can also use `_` inside another pattern to ignore just *part of a value*.

For example, when we want to test for only *part of a value* but have no use for the other parts in the corresponding code we want to run. 

```rs
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
```

This code will print `Can't overwrite an existing customized value` and then `setting is Some(5)`.

In the **first match arm**, we don’t need to match on or use the values inside either `Some` variant, but we do need to test for the case when `setting_value` and `new_setting_value` are the `Some` variant.

In that case, we print the reason for *not changing* `setting_value`, and it doesn’t get changed.

In all other cases (if either `setting_value` or `new_setting_value` is `None`) expressed by the `_` pattern in the second arm, we want to allow `new_setting_value` to become `setting_value`.

We can also use *underscores* in multiple places within one pattern to ignore **particular** values.

```rs
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
```

This code will print `Some numbers: 2, 8, 32`, and the values `4` and `16` will be **ignored**.

#### An Unused Variable by Starting Its Name with _

If you create a variable but *don’t use it anywhere*, Rust will usually issue a **warning** because an unused variable could be a bug.

However, sometimes it’s *useful* to be able to create a variable you won’t use yet, such as when you’re **prototyping** or just **starting a project**.

In this situation, you can tell Rust **not to warn** you about the unused variable by starting the name of the variable with an **underscore**.

```rs
fn main() {
    let _x = 5;
    let y = 10;
}
```

Here, we get a **warning** about not using the variable `y`, but we don’t get a **warning** about not using `_x`.

Note: the syntax `_x` *still binds* the value to the variable, whereas `_` doesn’t bind at all.

```rs
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{s:?}");
```

The above will return an error because the `s` value will still be moved into `_s`.

This prevents us from using `s` again. 

However, using the **underscore** by itself *doesn’t ever bind* to the value. 

```rs
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
```

This code works just fine because we never bind `s` to anything; **it isn’t moved**.

#### Remaining Parts of a Value with ..

With values that have many parts, we can use the `..` syntax to use specific parts and ignore the rest.

With this we can avoid the need to list *underscores* for **each** ignored value.

The `..` pattern ignores any parts of a value that we haven’t *explicitly matched* in the rest of the pattern. 

In the `match` expression, we want to operate only on the `x` coordinate and ignore the values in the `y` and `z` fields.

```rs
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }
```

We list the `x` value and then just include the `..` pattern.

This is **quicker** than having to list `y: _` and `z: _`.

The syntax `..` will expand to **as many values as it needs to** be. 

```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

In this code, the `first` and `last` values are matched with `first` and `last`. The `..` will match and ignore **everything in the middle**.

However, using `..` must be **unambiguous**. If it is *unclear* which values are intended for matching and which should be ignored, Rust will give us an **error**.

```rs
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {second}")
        },
    }
}
```

When we compile this example, we get this error:

```sh
$ cargo run
   Compiling patterns v0.1.0 (file:///projects/patterns)
error: `..` can only be used once per tuple pattern
 --> src/main.rs:5:22
  |
5 |         (.., second, ..) => {
  |          --          ^^ can only be used once per tuple pattern
  |          |
  |          previously used here

error: could not compile `patterns` (bin "patterns") due to 1 previous error
```

It’s **impossible** for Rust to determine how many values in the tuple to ignore before matching a value with second and then how many further values to ignore thereafter. 

#### Adding Conditionals with Match Guards

A `match guard` is an additional `if condition`, specified *after* the pattern in a match arm, that **must also match** for that arm to be chosen.

`Match guards` are useful for expressing **more complex** ideas than a pattern alone allows. 

Note, however, that they are **only available** in `match` expressions, not `if let` or `while let` expressions.

The **condition** can use variables *created in the pattern*.

Lets see a `match` where the **first arm** has the pattern `Some(x)` and also has a `match guard` of `if x % 2 == 0` (which will be true if the number is even):

```rs
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
```

This example will print `The number 4 is even`.

When `num` is compared to the pattern in the **first arm**, it matches because `Some(4)` matches `Some(x)`.

Then, the `match guard` checks whether the remainder of dividing `x by 2` is equal to `0`, and because it is, the **first arm is selected**.

If `num` had been `Some(5)` instead, the `match guard` in the **first arm** would have been `false` because the remainder of `5 divided by 2 is 1`, which is not equal to `0`. 

Rust would then go to the **second arm**, which would match because the **second arm** doesn’t have a `match guard` and therefore matches any `Some` variant.

There is no way to express the `if x % 2 == 0` condition within a pattern, so the `match guard` gives us the ability to express this logic.

The downside of this *additional expressiveness* is that the compiler **doesn’t** try to check for **exhaustiveness** when `match guard` expressions are involved.

Heres another example of the `match guard` in use:

```rs
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}
```

This code will print `Default case, x = Some(5)`.

The pattern in the **second match arm** doesn’t introduce a `new variable y` that would **shadow** the outer `y`, meaning we can use the outer `y` in the `match guard`.

Instead of specifying the pattern as `Some(y)`, which would have shadowed the outer `y`, we specify `Some(n)`. 

This creates a new variable `n` that **doesn’t shadow** anything because there is no `n` variable outside the match.

The `match guard`: `if n == y` is **not a pattern** and therefore *doesn’t introduce new variables*.

You can also use the or operator `|` in a `match guard` to specify **multiple patterns**; the `match guard` condition will apply to **all** the patterns. 

The important part of this follownig example is that the `if y match guard` applies to `4`, `5`, and `6`, even though it might look like if `y` only applies to `6`.

```rs
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
```

The `match` condition states that the arm **only matches** if the value of `x` is equal to `4`, `5`, or `6` and if `y is true`.

When this code runs, the pattern of the first arm matches because `x` is `4`, but the `match guard`: `if y` is `false`, so the **first arm** is not chosen.

The code moves on to the **second arm**, which does match, and this program prints `no`.

The reason is that the `if condition` applies to the **whole pattern** `4 | 5 | 6`, not just to the last value `6`

In other words, the **precedence** of a `match guard` in relation to a pattern behaves like this:

```(4 | 5 | 6) if y => ...```

Rather tha this:

```4 | 5 | (6 if y) => ...```

### Using @ Bindings

The `at` operator `@` lets us **create a variable** that holds a value at the **same time** we’re testing that value for a pattern match.

In the following example we want to test that a `Message::Hello` id field is within the range `3..=7`. 

We also want to *bind the value* to the variable `id` so that we can use it in the code associated with the arm.

```rs
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {id}"),
    }
```

This example will print `Found an id in range: 5`.

By specifying `id @` before the range `3..=7`, we’re capturing whatever value matched the range in a variable named `id` while also testing that the value matched the range pattern.

In the **second arm**, where we only have a range specified in the pattern, the code associated with the arm **doesn’t have** a variable that contains the actual value of the `id` field.

The pattern code **isn’t able to use** the value from the `id` field because we haven’t saved the `id` value in a variable.

In the **last arm**, where we’ve specified a variable without a range, we do have the value available to use in the arm’s code in a variable named `id`.

Using `@` lets us test a value and save it in a variable within one pattern.