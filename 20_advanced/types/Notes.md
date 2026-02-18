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

Rust has a special type named `!` that’s known in type theory lingo as the **empty type** because it has no values.

We prefer to call it the **never type** because it stands in the place of the return type when a function will **never** return.

```rs
fn bar() -> ! {
    // --snip--
}
```

This code is read as *“the function bar returns never.”*

Functions that return never are called **diverging functions**.

We **can’t create values** of the type `!`, so `bar` can never possibly return.

But what use is a type you can never create values for?

Let's recall the code for part of our previous number guessing game:

```rs
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
```

We had additionally discussed how match types must return the same type, as such the following will not compile:

```rs
    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };
```

The type of `guess` in this code would have to be an integer and a string, and Rust requires that guess have only one type.

So, what does `continue` return?

How were we allowed to return a `u32` from one arm and have another arm that ends with `continue`?

As you might have guessed, `continue` has a `!` value.

That is, when Rust computes the type of `guess`, it looks at **both** match arms, the former with a value of `u32` and the latter with a `!` value.

Because `!` can never have a value, Rust decides that the type of `guess` is `u32`.

The formal way of describing this behavior is that expressions of type `!` can be **coerced** into any other type.

We’re *allowed* to end this match arm with `continue` because `continue` doesn’t return a value.

Tnstead, `continue` moves control back to the top of the loop, so in the `Err` case, we **never** assign a value to `guess`.

The **never** type is useful with the `panic!` macro as well.

Recall the `unwrap` function that we call on `Option<T>` values to produce a value or panic with this definition:

```rs
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

In this code, the same thing happens as above:

Rust sees that `val` has the type `T` and `panic!` has the type `!`, so the result of the overall match expression is `T`.

This code works because `panic!` doesn’t produce a value; it ends the program.

In the `None` case, we won’t be returning a value from `unwrap`, so this code is valid.

One final expression that has the type `!` is a loop:

```rs
    print!("forever ");

    loop {
        print!("and ever ");
    }
```

Here, the loop never ends, so `!` is the value of the expression.

However, this *wouldn’t be true* if we included a `break`, because the `loop` would terminate when it got to the `break`.

#### Dynamically Sized Types and the Sized Trait

Rust needs to know certain details about its types, such as *how much space* to allocate for a value of a particular type.

This leaves one corner of its type system a little confusing at first: the concept of **dynamically sized types**.

Sometimes referred to as `DSTs` or `unsized types`, these types let us write code using values whose size we can know only at **runtime**.

Let’s dig into the details of a very familiar **dynamically sized type** called `str`.

That’s right, not `&str`, but `str` on its own, is a **DST**.

In many cases, such as when storing text entered by a user, we **can’t know** how long the string is until **runtime**.

That means we can’t create a variable of type `str`, nor can we take an argument of type `str`. 

Consider the following code, which **does not work**:

```rs
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
```

Rust needs to know how much memory to allocate for any value of a particular type, and **all values of a type must use the same amount of memory**.

**If** Rust allowed us to write this code, these two `str` values would need to take up the same amount of space. 

But they have different lengths: `s1` needs **12** bytes of storage and `s2` needs **15**.

This is why it’s **not possible** to create a variable holding a **dynamically sized type**.

So, what do we do? In this case, you already know the answer: 

We make the type of `s1` and `s2` string slice (&str) rather than `str`.

Recall that the slice data structure only stores the starting position and the length of the slice.

So, although `&T` is a single value that stores the memory address of where the `T` is located, a string slice is two values: the address of the `str` and its **length**.

As such, we can know the size of a string slice value at compile time: It’s twice the length of a `usize`.

That is, we always know the size of a string slice, no matter how long the string it refers to is.

In general, this is the way in which *dynamically sized types* are used in Rust: They have an extra bit of metadata that stores the size of the dynamic information.

The **golden rule** of *dynamically sized types* is that we must always put values of *dynamically sized types* **behind** a pointer of some kind.

We can combine str with all kinds of pointers: for example, `Box<str>` or `Rc<str>`.

In fact, you’ve seen this before but with a different dynamically sized type: **traits**. **Every trait** is a *dynamically sized type* we can refer to by using the name of the trait.

Prevously, we mentioned that to use **traits** as `trait objects`, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn Trait>` would work too).

To work with **DSTs**, Rust provides the `Sized` trait to determine whether or not a type’s size is known at compile time. 

This trait is **automatically** implemented for everything whose size is known at compile time. 

In addition, Rust *implicitly* adds a bound on `Sized` to every generic function.

That is, a generic function definition like this:

```rs
fn generic<T>(t: T) {
    // --snip--
}
```

is actually treated as though we had written this:

```rs
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will work only on types that have a known size at compile time.

However, you can use the following special syntax to relax this restriction:

```rs
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

A trait bound on `?Sized` means *“T may or may not be Sized,”* and this notation overrides the default that generic types must have a known size at compile time.

The `?Trait` syntax with this meaning is only available for `Sized`, not any other traits.

Also note that we switched the type of the `t` parameter from `T` to `&T`.

Because the type might not be `Sized`, we need to use it behind some kind of pointer.