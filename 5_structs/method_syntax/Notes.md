## Chapter 5 – Method Syntax

### `impl` Blocks and Methods
- Methods are defined in `impl` blocks and take `&self` or `&mut self`.

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### Associated Functions
- Associated functions don’t take `self` and are called with `::`.

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

### Extra Notes (from chapter summary)
- To define the methods of a struct we use an `impl` block.
- Methods that only read data take `&self`; methods that mutate data take `&mut self`.
- A struct can have multiple `impl` blocks; this is mostly for organization.


