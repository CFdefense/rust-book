## Chapter 5 – Defining Structs

### Basic Struct Definition
- Structs group related data together under one name.
- Fields have names and types.

```rs
struct User {
    username: String,
    email: String,
    active: bool,
}
```

### Creating and Mutating Values
- The whole instance must be `mut` to change any field.

```rs
let mut user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    active: true,
};

user.email = String::from("alice@new.com");
```

### Extra Notes (from chapter summary)
- Structs must be `mut` to be able to change any of their associated values.
- There are three types of structs: normal structs, tuple structs, and unit structs.
- You can copy fields during initialization from another struct of the same type using the shorthand `..`.
- It’s possible for structs to store references to data owned by something else, but to do so requires lifetimes.


