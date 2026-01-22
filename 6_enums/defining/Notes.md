## Chapter 6 – Defining Enums

### Basic Enum Definition
- Enums define a type by listing its possible variants.

```rs
enum IpAddrKind {
    V4,
    V6,
}

let home = IpAddrKind::V4;
```

### Variants with Data
- Variants can hold data like structs or tuples.

```rs
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

### Extra Notes (from chapter summary)
- Just as we can define methods for structs using `impl`, we can do so for enums.
- The `Option` enum is used to represent values that may be null/absent.
- You cannot directly add to an `Option` because it might be `None`.


