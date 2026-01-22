## Chapter 8 – Vectors

### Creating and Updating Vectors
- Vectors store a growable list of values on the heap.

```rs
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
```

### Accessing Elements
- Indexing panics on out-of-bounds; `get` returns an `Option`.

```rs
let third = v[2];
let maybe_fourth = v.get(3);
```

### Extra Notes (from chapter summary)
- We can reference a value stored in a vector by either its index or by using the `get` method.
- We can have multiple types of values in one vector using enums.
- When a vector is dropped, so are the values inside it.


