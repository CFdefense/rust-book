## Chapter 13 – Iterators

### Creating Iterators
- Use `.iter()`, `.iter_mut()`, or `.into_iter()` on collections.

```rs
let v = vec![1, 2, 3];
for value in v.iter() {
    println!("{value}");
}
```

### Adapter and Consumer Methods
- Adapters create new iterators; consumers run the iteration and produce a result.

```rs
let sum: i32 = v.iter().sum();
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
```

### Extra Notes (from chapter summary)
- The iterator pattern performs some task on a sequence of items in turn; the iterator is responsible for the logic of iterating and determining when we’re done.
- Rust iterators are lazy, meaning they have no effect until we call methods that consume the iterator (simply calling `.iter()` by itself does nothing).
- Iterators handle much of the boilerplate of initializing and advancing through collections.
- The `Iterator` trait only requires implementors to define one method: `next`, which returns items wrapped in `Some` and `None` when iteration is over.
- `.iter()` returns an iterator over immutable references; if we want ownership we use `.into_iter()`; for mutable references we use `.iter_mut()`.
- Many methods with default implementations can be used on iterator types.
- Methods that call `next` and consume the iterator are called consuming adapters.
- Methods that transform an iterator into another iterator without consuming it are called iterator adapters.

