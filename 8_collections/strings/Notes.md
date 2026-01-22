## Chapter 8 – Strings

### `String` vs `&str`
- `String` is an owned, growable UTF-8 string.
- `&str` is a borrowed string slice.

```rs
let mut s = String::from("hello");
s.push_str(" world");
let slice: &str = &s;
```

### Indexing and Slices
- You can’t index `String` by integer position because of UTF-8.
- Use slices carefully: indices must fall on character boundaries.

### Extra Notes (from chapter summary)
- Rust only has one string data type at its core: the `str` slice.
- The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
- When indexing a `String` it is important to specify whether we’re indexing bytes or characters.


