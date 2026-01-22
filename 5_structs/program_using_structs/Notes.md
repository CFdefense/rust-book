## Chapter 5 – Program Using Structs

### Using Structs in a Small Program
- Structs help make program state clearer and easier to work with.

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

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area is {} square pixels", rect.area());
}
```

### Extra Notes (from chapter summary)
- Structs can be combined with functions and methods to build larger programs.


