## Chapter 3 – Functions

### Defining Functions
- Functions use **snake_case** names and the `fn` keyword.
- Parameters must have **explicit types**.

```rs
fn main() {
    println!("In main");
    print_number(10);
}

fn print_number(x: i32) {
    println!("x is {x}");
}
```

### Statements vs Expressions
- **Statements**: perform an action, do not return a value (e.g. `let y = 5;`).
- **Expressions**: produce a value and can be part of statements.

```rs
let y = {
    let x = 3;
    x + 1 // expression, no semicolon
};
println!("y = {y}");
```

### Return Values
- The last expression in a function body is the return value.
- Use `return` early for clarity when needed.

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Extra Notes (from chapter summary)
- Functions in Rust must use snake case and can be declared anywhere within the scope that they are being used.
- Statements are instructions that perform some action and do not return a value (for example, `let y: u32 = 10;`).
- Expressions evaluate to a resultant value.


