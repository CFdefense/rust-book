## Chapter 20 - Advanced Features: Advanced Types

### Summary

The Rust type system has some features that we’ve so far mentioned but haven’t yet discussed. We’ll start by discussing `newtypes` in general as we examine why they are useful as types. Then, we’ll move on to `type aliases`, a feature similar to newtypes but with slightly different semantics. We’ll also discuss the `!` type and `dynamically sized types`.

#### Type Safety and Abstraction with the Newtype Pattern

The _newtype pattern_ is also useful for tasks beyond those we’ve discussed so far, including **statically** enforcing that values are never confused and indicating the units of a value.

We used _newtypes_ to indicate units in a previous example.

Recall that the `Millimeters` and `Meters` structs wrapped `u32` values in a _newtype_.

If we wrote a function with a parameter of type `Millimeters`, we wouldn’t be able to compile a program that **accidentally** tried to call that function with a value of type `Meters` or a plain `u32`.

We can also use the _newtype pattern_ to abstract away some implementation details of a type:

The new type can expose a public API that is **different** from the API of the private inner type.

**Newtypes** can also hide internal implementation.

For example, we could provide a `People` type to wrap a `HashMap<i32, String> `that stores a person’s ID associated with their name.

Code using `People` would only interact with the public API we provide, such as a method to **add** a name string to the `People` collection; that code wouldn’t need to know that we assign an i32 ID to names _internally_.

The _newtype pattern_ is a lightweight way to achieve encapsulation to hide implementation detail.

#### Type Synonyms and Type Aliases

Rust provides the ability to declare a `type alias` to give an existing type another name.

For this we use the `type` keyword. For example, we can create the alias `Kilometers` to `i32` like so:

```rs
    type Kilometers = i32;
```

Now the _alias_ `Kilometers` is a synonym for `i32`; unlike the `Millimeters` and `Meters` types we created before, `Kilometers` is not a separate, new type.

Values that have the type `Kilometers` will be treated the **same** as values of type `i32`:

```rs
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
```

Because `Kilometers` and `i32` are the same type, we can add values of both types and can pass `Kilometers` values to functions that take `i32` parameters.

However, using this method, we **don’t** get the type-checking benefits that we get from the _newtype pattern_ discussed earlier.

In other words, if we mix up `Kilometers` and `i32` values somewhere, the compiler will **not** give us an error.

The main use case for type synonyms is to _reduce repetition_. For example, we might have a lengthy type like this:

```rs
Box<dyn Fn() + Send + 'static>
```

Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error-prone.

Imagine having a project full of code like this:

```rs
let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
}
```

A type alias makes this code more manageable by reducing the repetition. Lets replace the type with the _alias_ `Thunk`:

```rs
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

This code is much easier to read and write! Choosing a _meaningful_ name for a type alias can help communicate your intent as well

`Thunk` is a word for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored.

_Type aliases_ are also commonly used with the `Result<T, E>` type for reducing repetition.

Consider the `std::io` module in the standard library.

I/O operations often return a `Result<T, E>` to handle situations when operations fail to work.

This library has a `std::io::Error` struct that represents all possible I/O errors.

Many of the functions in `std::io` will be returning `Result<T, E>` where the `E` is `std::io::Error`, such as these functions in the `Write` trait:

```rs
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
```

The `Result<..., Error>` is repeated a lot. As such, `std::io` has this type alias declaration:

```rs
type Result<T> = std::result::Result<T, std::io::Error>;
```

Because this declaration is in the `std::io` module, we can use the fully qualified alias `std::io::Result<T>`.

That is, a `Result<T, E>` with the `E` filled in as `std::io::Error`. The `Write` trait function signatures end up looking like this:

```rs
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

The type alias helps in two ways: It makes code easier to write and it gives us a consistent interface across all `of std::io`.

Because it’s an _alias_, it’s just another `Result<T, E>`, which means we can use any methods that work on `Result<T, E>` with it, as well as special syntax like the `?` operator.

#### The Never Type That Never Returns
