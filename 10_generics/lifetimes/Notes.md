## Chapter 10 – Lifetimes

### Function Lifetime Annotations
- Lifetimes describe how long references are valid.

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### Structs with References
- Structs that hold references need lifetime parameters too.

```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

### Extra Notes (from chapter summary)
- The main aim of lifetimes is to prevent dangling references.
- The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
- Lifetimes refer to the scope in which a reference is valid; Rust will require annotations when it cannot infer them.
- Structs, functions, variables, and more can be declared with lifetime annotations.
- Lifetime annotations do not change how long references live; they describe relationships the compiler enforces.
- Lifetime elision rules are patterns the compiler uses to allow you to skip explicit lifetimes in many cases.
- Lifetimes on parameters are input lifetimes; lifetimes on return values are output lifetimes.
- The compiler uses three rules to infer lifetimes; if it still can’t figure them out, it stops with an error.
- Example rules:
  - Each reference parameter gets its own lifetime parameter.
  - If there is exactly one input lifetime, that lifetime is assigned to all output lifetimes.
  - If there are multiple input lifetimes and one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetimes.

