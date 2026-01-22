## Chapter 4 – Referencing and Borrowing

### References
- References let you **borrow** a value without taking ownership.
- Immutable references: multiple allowed at the same time.
- Mutable reference: only **one mutable reference** at a time to a value.

```rs
let s = String::from("hello");
let len = calculate_length(&s); // borrow s

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

### Mutable References and Aliasing Rules
- You cannot have a mutable reference while immutable references to the same value exist.
- These rules prevent **data races** at compile time.

### Extra Notes (from chapter summary)
- Using referencing, we can pass values into functions without giving up ownership.
- Immutable references cannot be used to change the value they point to (unless you use a mutable reference).
- Mutable references have one big restriction: you can only have one mutable reference to a value at a time.
- We cannot have a mutable and immutable reference to the same value simultaneously.
- Rust will not allow us to reference a value in memory that is no longer there (no dangling pointers).
