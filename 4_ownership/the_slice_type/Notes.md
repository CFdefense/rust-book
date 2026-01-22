## Chapter 4 – The Slice Type

### String Slices
- Slices reference a **portion** of a collection without taking ownership.
- `&str` is a string slice; `String` can give slices into its data.

```rs
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..];
```

### Slices and Safety
- Slices keep track of **length**, preventing out-of-bounds access.
- Functions can take `&str` to work with both string literals and `String` values.

### Extra Notes (from chapter summary)
- Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
- A slice is a kind of reference, so it does not have ownership.
- String slices are good to use to maintain data integrity and avoid dangling pointers.

