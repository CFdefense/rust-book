## Chapter 10 – Generics

### Generic Functions
- Generics let you reuse logic for multiple types.

```rs
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### Generic Structs and Enums
- You can define types that hold generic parameters.

```rs
struct Point<T> {
    x: T,
    y: T,
}
```

### Extra Notes (from chapter summary)
- Generics allow us to reuse functions and logic for multiple types.
- When defining a function that uses generics, we place the generics in the function signature.
- If we want to find the largest of some type and don’t use generics, we’d need to repeat code for each type.
- We cannot compare all types with `>`; only those which implement `std::cmp::PartialOrd` (traits and generics work hand in hand).
- We can declare generic types, function types, method types, struct types, and more.

