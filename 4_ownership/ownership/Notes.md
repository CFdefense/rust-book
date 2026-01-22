## Chapter 4 – Ownership Basics

### Ownership Rules
- Each value in Rust has a **single owner**.
- There can only be **one owner at a time**.
- When the owner goes **out of scope**, the value is **dropped**.

```rs
{
    let s = String::from("hello"); // s owns the String
    // use s here
} // s goes out of scope, String is dropped
```

### Move vs Clone
- Assignment or passing to a function **moves** ownership for non-`Copy` types.
- Use `.clone()` when you need a **deep copy**.

```rs
let s1 = String::from("hello");
let s2 = s1;        // move, s1 is invalid
let s3 = s2.clone(); // deep copy, both valid
```

### Extra Notes (from chapter summary)
- Ownership is a set of rules that govern how a Rust program manages memory.
- Ownership rules:
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- Simple values like integers are cheap to move.
- For `String` and other heap data, a move transfers the pointer, length, and capacity, invalidating the previous owner.
- Types that implement the `Copy` trait are never moved; they are copied instead.
- Types that implement `Copy` by default include:
  - All integer types, such as `u32`.
  - The boolean type, `bool`.
  - All floating-point types, such as `f64`.
  - The character type, `char`.
  - Tuples, if all fields implement `Copy`.


