## Chapter 20 - Advanced Features: Advanced Functions and Closures

### Summary

This section explores some advanced features related to functions and closures, including function pointers and returning closures.

#### Function Pointers

We’ve talked about how to pass `closures` to `functions`; you can also pass regular `functions` to `functions`!

This technique is useful when you want to pass a `function` _you’ve already defined_ rather than defining a new `closure`.

Functions coerce to the type `fn` (with a lowercase f), not to be confused with the `Fn` closure trait.

The `fn` type is called a **function pointer**.

Passing functions with function pointers will allow you to use functions as **arguments** to other functions.

The syntax for specifying that a parameter is a function pointer is similar to that of closures, as shown below, where we’ve defined a function `add_one` that adds 1 to its parameter.

The function `do_twice` takes two parameters: a function pointer to any function that takes an `i32` parameter and returns an `i32`, and one `i32` value.

The `do_twice` function calls the function `f` twice, passing it the arg value, then adds the two function call results together.

The main function calls `do_twice` with the arguments `add_one` and `5`.

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");
}
```

This code prints _The answer is: 12_.

We specify that the parameter `f` in `do_twice` is an `fn` that takes one parameter of type `i32` and returns an `i32`.

We can then call `f` in the body of `do_twice`. In main, we can pass the function name `add_one` as the first argument to `do_twice`.

Unlike `closures`, `fn` is a **type** rather than a **trait**, so we specify `fn` as the parameter type directly rather than declaring a generic type parameter with one of the `Fn` traits as a trait bound.

**Function pointers** implement all three of the closure traits (`Fn`, `FnMut`, and `FnOnce`), meaning you can **always** pass a **function pointer** as an argument for a function that expects a `closure`.

It’s best to write functions using a generic type and one of the closure traits so that your functions can accept either functions or closures.

That said, one example of where you would want to **only** accept `fn` and **not** closures is when interfacing with **external code** that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

As an example of where you could use either a **closure** defined inline or a **named function**, let’s look at a use of the `map` method provided by the `Iterator` trait in the standard library.

To use the `map` method to turn a `vector` of numbers into a `vector` of strings, we could use a `closure`:

```rs
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
```

Or we could name a `function` as the argument to `map` instead of the `closure`:

```rs
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```

Note that we must use the **fully qualified syntax** that we talked about in the “Advanced Traits” section because there are **multiple** functions available named `to_string`.

Here, we’re using the `to_string` function defined in the `ToString` trait, which the standard library has implemented for any type that implements `Display`.

Recall from the “Enum Values” section in Chapter 6 that the name of **each** `enum` variant that we define also becomes an **initializer function**.

We can use these **initializer functions** as **function pointers** that implement the `closure` traits

This means we can specify the **initializer functions** as arguments for methods that take `closures`:

```rs
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

Here, we create `Status::Value` instances using each `u32` value in the range that `map` is called on by using the **initializer function** of `Status::Value`.

Some people prefer this style and some people prefer to use closures.

They compile to the same code, so use whichever style is clearer to you.

#### Returning Closures

`Closures` are represented by `traits`, which means you **can’t** return closures directly.

In most cases where you might want to **return a trait**, you can **instead** use the **concrete type** that implements the trait as the return value of the function.

However, you **can’t** usually do that with `closures` because they don’t have a concrete type that is returnable;

You’re **not allowed** to use the function pointer `fn` as a return type if the `closure` captures any values from its scope, for example.

Instead, you will normally use the `impl Trait` syntax we learned about in Chapter 10.

You can return any function type, using `Fn`, `FnOnce`, and `FnMut`.

For example, the code below will compile just fine.

```rs
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}
```

However, as we noted in the “Inferring and Annotating Closure Types” section in Chapter 13, **each** closure is also its _own distinct type_.

If you need to work with **multiple** functions that have the same signature but different implementations, you will need to use a **trait object** for them.

Consider what happens if you write code like that shown below:

```rs
fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}
```

Here we have two functions, `returns_closure` and `returns_initialized_closure`, which both return `impl Fn(i32) -> i32`.

Notice that the `closures` that they return are **different**, even though they implement the same type.

If we try to compile this, Rust lets us know that it _won’t work_:

```sh
$ cargo build
   Compiling functions-example v0.1.0 (file:///projects/functions-example)
error[E0308]: mismatched types
  --> src/main.rs:2:44
   |
 2 |     let handlers = vec![returns_closure(), returns_initialized_closure(123)];
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
...
 9 | fn returns_closure() -> impl Fn(i32) -> i32 {
   |                         ------------------- the expected opaque type
...
13 | fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
   |                                              ------------------- the found opaque type
   |
   = note: expected opaque type `impl Fn(i32) -> i32`
              found opaque type `impl Fn(i32) -> i32`
   = note: distinct uses of `impl Trait` result in different opaque types

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions-example` (bin "functions-example") due to 1 previous error
```

The error message tells us that whenever we return an `impl Trait`, Rust creates a **unique opaque type**, a type where we **cannot** see into the details of what Rust constructs for us, nor can we guess the type Rust will generate to write ourselves.

So, even though these functions return closures that implement the **same** trait, `Fn(i32) -> i32`, the **opaque types** Rust generates for each are _distinct_.

We have seen a solution to this problem a few times now: We can use a **trait object**:

```rs
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
```

Above we create a `Vec<T>` of `closures` defined by functions that return `Box<dyn Fn>` so that they have the **same** type.

This code will compile just fine!
