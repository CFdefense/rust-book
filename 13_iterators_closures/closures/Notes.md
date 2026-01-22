## Chapter 13 – Closures

### Defining Closures
- Closures are anonymous functions that can capture their environment.

```rs
let add_one = |x: i32| x + 1;
let result = add_one(5);
```

### Capturing Environment
- Closures infer whether to borrow immutably, mutably, or take ownership.

```rs
let x = String::from("hello");
let print_x = || println!("{x}");
print_x();
```

### Extra Notes (from chapter summary)
- Closures are anonymous functions in Rust.
- They can be assigned to variables and used later on.
- Closures implicitly decide whether to mutably or immutably borrow or take ownership of arguments based on what happens in the body.
- You can specify a closure to take ownership of captured values explicitly using `move`.
- The way a closure captures and handles values from the environment affects which `Fn` traits the closure implements.
- Closures will automatically implement one, two, or all three of the `Fn` traits in an additive fashion, depending on how the closure’s body handles the values.
- `FnOnce` applies to closures that can be called once and may move captured values out of the body.
- `FnMut` applies to closures that don’t move captured values out of their body but might mutate captured values; they can be called more than once.
- `Fn` applies to closures that don’t move or mutate captured values, as well as closures that capture nothing from their environment.

