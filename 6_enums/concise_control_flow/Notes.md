## Chapter 6 – Concise Control Flow with `if let`

### `if let` Syntax
- Combines `if` and `let` to match one pattern and ignore the rest.

```rs
let some_value = Some(3);

if let Some(3) = some_value {
    println!("three");
} else {
    println!("not three");
}
```

### When to Use
- Use `if let` when you only care about a **single** match arm and want less boilerplate than `match`.

### Extra Notes (from chapter summary)
- The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.



