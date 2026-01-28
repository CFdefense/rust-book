## Chapter 18 - Object-Oriented Programming Features: Using Trait Objects to Abstract over Shared Behavior

### Overview

Sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.

To show how we might achieve this, we’ll create an example GUI tool that iterates through a list of items, calling a `draw` method on each one to draw it to the screen—a common technique for GUI tools.

We’ll create a library crate called `gui` that contains the structure of a `GUI` library.

This crate might include some types for people to use, such as `Button` or `TextField`.

In addition, gui users will want to *create their own types* that can be drawn.

For instance, one programmer might add an `Image`, and another might add a `SelectBox`.

At the time of writing the library, we *can’t* know and define all the types other programmers might want to create.

But we do know that gui needs to keep track of many values of different types, and it needs to call a `draw` method on each of these differently typed values.

It doesn’t need to know exactly what will happen when we call the `draw` method, *just* that the value will have that method available for us to call.

To do this in a language **with inheritance**, we might define a `class` named `Component` that has a `method` named `draw` on it. 

The other classes, such as `Button`, `Image`, and `SelectBox`, would **inherit** from `Component` and thus **inherit** the `draw` method.

They could each *override* the `draw` method to define their custom behavior, but the framework could treat all of the types as if they were `Component` instances and call `draw` on them.

But because Rust **doesn’t have inheritance**, we need another way to structure the gui library to allow users to create new types compatible with the library.

### Defining a Trait for Common Behavior

To implement the behavior that we want gui to have, we’ll define a `trait` named `Draw` that will have one method named `draw`. 

Then, we can define a vector that takes a `trait object`.

A `trait object` points to *both* an instance of a type implementing our specified `trait` and a table used to look up trait methods on that type at runtime. 

We create a `trait object` by specifying some sort of pointer, such as a `reference` or a `Box<T> smart pointer`, then the `dyn` keyword, and then specifying the relevant `trait`. 

We can use `trait objects` *in place* of a `generic` or `concrete type`.

Wherever we use a `trait object`, Rust’s type system will *ensure* at compile time that any value used in that context will implement the trait object’s `trait`. 

Consequently, we *don’t* need to know all the possible types at compile time.

#### Rust 'objects'

In Rust, we refrain from calling `structs` and `enums` “objects” to distinguish them from other languages’ `objects`.

In a `struct` or `enum`, the data in the `struct` fields and the behavior in `impl blocks` are **separated**.

Whereas in other languages, the *data* and *behavior* are commonly combined into one concept is often labeled an `object`.

`Trait objects` differ from `objects` in other languages in that we *can’t* add data to a `trait object`.

`Trait objects` aren’t as generally useful as `objects` in other languages: Their specific purpose is to *allow abstraction* across common behavior.

#### Trait Objects

Lets begin by defining our `Draw` trait:

```rs
pub trait Draw {
    fn draw(&self);
}
```

Now lets implement `Screen` a `struct` which will control the componenets of our gui:

```rs
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

This vector `components` is of type `Box<dyn Draw>`, which is a `trait object`; it’s a stand-in for any type inside a `Box` that implements the `Draw` trait.

On the `Screen` struct, we’ll define a method named `run` that will call the `draw` method on each of its `components`:

```rs
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This works differently from defining a `struct` that uses a `generic type parameter` with `trait bounds`.

A `generic type parameter` can be substituted with only *one concrete type at a time*, whereas `trait objects` allow for *multiple concrete types* to fill in for the `trait object` at **runtime**. 

For example, we could have defined the `Screen` struct using a `generic type` and a `trait bound`:

```rs
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This restricts us to a `Screen` instance that has a list of components **all** of type `Button` or **all** of type `TextField`.

If you’ll only ever have `homogeneous collections`, using `generics` and `trait bounds` is **preferable** because the definitions will be `monomorphized` at `compile time` to use the `concrete types`.

On the other hand, with the method using `trait objects`, one `Screen` instance can hold a `Vec<T> `that contains a `Box<Button>` as well as a `Box<TextField>`.

Let’s look at how this works, and then we’ll talk about the runtime performance implications.

### Implementing the Trait

Now we’ll add some types that implement the `Draw` trait. Lets start with the `Button` type. 

To imagine what the implementation might look like, a `Button` struct might have fields for `width`, `height`, and `label`:

```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

The `width`, `height`, and `label` fields on `Button` will *differ* from the fields on other components; 

For example, a `TextField` type might have those same fields plus a `placeholder` field. 

Each of the types we want to draw on the screen will implement the `Draw` trait but will use different code in the `draw` method to define how to draw that particular type.

The `Button` type, for instance, might have an additional `impl block` containing methods related to what happens when a user clicks the button.

These kinds of methods won’t apply to types like `TextField`.

If someone using our library decides to implement a `SelectBox` struct that has `width`, `height`, and `options` fields.

They would implement the `Draw` trait on the `SelectBox` type as well:

```rs
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
```

Our library’s user can now write their `main` function to create a `Screen` instance.

To the `Screen` instance, they can add a `SelectBox` and a `Button` by putting each in a `Box<T>` to become a `trait object`. 

They can then call the `run` method on the `Screen` instance, which will call `draw` on each of the `components`.

```rs
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

When we wrote the library, we didn’t know that someone might add the `SelectBox` type.

Despite this, the `Screen` implementation was able to operate on the new type and draw it because `SelectBox` implements the `Draw` trait, which means it implements the `draw` method.

This concept of being concerned only with **the messages a value responds to** *rather than* the **value’s concrete type** is similar to the concept of **duck typing** in dynamically typed languages.

**duck typing**: If it walks like a duck and quacks like a duck, then it must be a duck!

In our implementation of `run` on `Screen`, `run` doesn’t need to know what the concrete type of each component is.

It doesn’t check whether a `component` is an instance of a `Button` or a `SelectBox`, it *simply* calls the `draw` method on the `component`.

By specifying `Box<dyn Draw>` as the type of the values in the `components` vector, we’ve defined `Screen` to need values that we can call the `draw` method on.

The advantage of using `trait objects` and Rust’s type system to write code similar to code using duck typing is that we *never have to check* whether a value implements a particular method at `runtime`.

For example if we try passing a `String` as a `component` into the `vector` of `trait objects` we will get a compilation error as `String` does not implement `Draw`:

```sh
$ cargo run
   Compiling gui v0.1.0 (file:///projects/gui)
error[E0277]: the trait bound `String: Draw` is not satisfied
 --> src/main.rs:5:26
  |
5 |         components: vec![Box::new(String::from("Hi"))],
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
  |
  = help: the trait `Draw` is implemented for `Button`
  = note: required for the cast from `Box<String>` to `Box<dyn Draw>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `gui` (bin "gui") due to 1 previous error
```

This error lets us know that either we’re passing something to `Screen` that we didn’t mean to pass and so should pass a different type.

Or we should implement `Draw` on `String` so that `Screen` is able to call `draw` on it.

### Performing Dynamic Dispatch

Recall from Chapter 10 how the compiler preforms `monomorphization` on generics:

The compiler generates **nongeneric implementations** of functions and methods for each concrete type that we use in place of a `generic type parameter`.

The code that results from `monomorphization` is doing `static dispatch`, which is when the compiler **knows** what method you’re calling at **compile time**.

This is opposed to `dynamic dispatch`, which is when the compiler **can’t tell** at **compile time** which method you’re calling.

In `dynamic dispatch` cases, the compiler emits code that at **runtime** *will know* which method to call.

When we use `trait objects`, Rust must use `dynamic dispatch`.

The compiler doesn’t *know all* the types that might be used with the code that’s using `trait objects`, so it *doesn’t know* which method implemented on which type to call.

Instead, at **runtime**, Rust uses the pointers *inside* the `trait object` to know which method to call.

This lookup incurs a **runtime cost** that doesn’t occur with `static dispatch`.

`Dynamic dispatch` also *prevents* the compiler from choosing to inline a method’s code, which in turn *prevents* some optimizations.

Rust also has some rules about *where you can and cannot* use `dynamic dispatch`, called `dyn compatibility`.