## Chapter 20 - Advanced Features: Advanced Traits

### Summary

This section will focus on a deeper dive into Traits.

### Defining Traits with Associated Types

**Associated types** connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

The implementor of a trait will *specify the concrete type* to be used instead of the placeholder type for the particular implementation.

That way, we can define a trait that uses some types *without needing to know exactly what those types are* until the trait is implemented.

We’ve described most of the advanced features in this chapter as being *rarely needed*.

**Associated Types** however are more widely used than some of the other examples in this chapter.

One example of a `trait` with an **associated type** is the `Iterator` trait that the standard library provides. 

The associated type is named `Item` and stands in for the type of the values the type implementing the `Iterator` trait is iterating over.

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The type `Item` is a *placeholder*, and the next method’s definition shows that it will return values of type `Option<Self::Item>`.

Implementors of the `Iterator` trait will specify the concrete type for `Item`, and the `next` method will return an `Option` containing a value of that concrete type.

**Associated types** might seem like a similar concept to **generics**, in that the latter allow us to define a function without specifying what types it can handle.

To examine the **difference** between the two concepts, we’ll look at an implementation of the `Iterator` trait on a type named `Counter` that specifies the `Item` type is `u32`:

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

This syntax *seems* comparable to that of **generics**. So, why not just define the `Iterator` trait with **generics**, Like below?

```rs
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using **generics**, we must annotate the types in each implementation, because we can also implement `Iterator<String>` for `Counter` or any other type, we could have multiple implementations of `Iterator` for `Counter`.

In other words, when a **trait** has a **generic parameter**, it can be implemented for a type *multiple times*, changing the concrete types of the generic type parameters each time.

When we use the `next` method on `Counter`, we would have to provide type annotations to indicate **which** implementation of `Iterator` we want to use.

With **associated types**, we don’t need to annotate types, because we can’t implement a **trait** on a type multiple times. 

With the definition that uses **associated types**, we can choose what the type of `Item` will be only once because there can be only one `impl Iterator for Counter`.

We don’t have to specify that we want an `iterator` of `u32` values everywhere we call `next` on `Counter`.

**Associated types** also become part of the trait’s *contract*: Implementors of the trait must provide a type to stand in for the **associated type** placeholder.

### Using Default Generic Parameters and Operator Overloading

When we use generic type parameters, we can specify a **default concrete type** for the generic type.

This **eliminates** the need for implementors of the trait to specify a **concrete type** if the default type works.

You specify a **default type** when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax.

A great example of a situation where this technique is useful is with **operator overloading**.

**Operator Overloading** is when you customize the behavior of an operator (such as +) in particular situations.

Rust **doesn’t allow** you to create your own operators or overload arbitrary operators.

But you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator.

For example: we overload the `+` operator to add two `Point` instances together. 

We do this by implementing the `Add` trait on a `Point` struct.

```rs
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

The `add` method adds the `x` values of two `Point` instances and the `y` values of two `Point` instances to create a new `Point`.

The `Add` trait has an associated type named `Output` that determines the type returned from the `add` method.

The **default generic type** in this code is within the `Add` trait. Here is its definition:

```rs
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

This code should look generally familiar: a `trait` with one method and an **associated type**.

The new part is `Rhs=Self`: This syntax is called **default type parameters**.

The `Rhs` generic type parameter (short for “right-hand side”) defines the type of the `rhs` parameter in the `add` method.

If we don’t specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will default to `Self`, which will be the type we’re implementing `Add` on.

When we implemented `Add` for `Point`, we used the default for `Rhs` because we wanted to add two `Point` instances.

Let’s look at an example of implementing the `Add` trait where we want to customize the `Rhs` type rather than using the default.

We have two structs, `Millimeters` and `Meters`, holding values in different units.

This thin wrapping of an existing type in another struct is known as the **newtype pattern**.

We want to add values in `millimeters` to values in `meters` and have the implementation of `Add` do the conversion **correctly**.

We can implement `Add` for `Millimeters` with `Meters` as the `Rhs`:

```rs
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

To add `Millimeters` and `Meters`, we specify `impl Add<Meters>` to set the value of the `Rhs` type parameter instead of using the default of `Self`.

You’ll use **default type parameters** in two main ways:

1. To extend a type without breaking existing code
2. To allow customization in specific cases most users won’t need

The standard library’s `Add` trait is an example of the second purpose: Usually, you’ll add two like types, but the `Add` trait provides the ability to customize beyond that.

Using a **default type parameter** in the `Add` trait definition means you don’t have to specify the extra parameter most of the time.

In other words, a bit of implementation boilerplate isn’t needed, making it easier to use the trait.

The first purpose is similar to the second but in **reverse**: 

If you want to add a type parameter to an existing trait, you can give it a default to allow extension of the functionality of the trait without breaking the existing implementation code.

### Disambiguating Between Identically Named Methods

Nothing in Rust prevents a trait from having a method with the **same name** as another trait’s method, nor does Rust prevent you from implementing both traits on one type. 

It’s also possible to implement a method directly on the type with the **same name** as methods from traits.

When calling methods with the same name, you’ll need to tell Rust *which one* you want to use.

Consider the following code where we have `Pilot` and `Wizard`, that both have a method called `fly`.

We then implement both traits on a type `Human` that already has a method named `fly` implemented on it.

Each `fly` method does something different.

```rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

When we call `fly` on an instance of `Human`, the compiler **defaults** to calling the method that is directly implemented on the type:

```rs
fn main() {
    let person = Human;
    person.fly();
}
```

Running this code will print *waving arms furiously*, showing that Rust called the `fly` method implemented on `Human` directly.

To call the `fly` methods from either the `Pilot` trait or the `Wizard` trait, we need to use **more explicit syntax** to specify which `fly` method we mean.

```rs
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

Specifying the **trait name** before the method name clarifies to Rust which implementation of `fly` we want to call. 

We could also write `Human::fly(&person)`, which is equivalent to the `person.fly()`.

As a result the code now prints:

```sh
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/traits-example`
This is your captain speaking.
Up!
*waving arms furiously*
```

Because the `fly` method takes a `self` parameter, if we had two types that both implement one trait, Rust could figure out which implementation of a trait to use based on the type of `self`.

However, **associated functions** that are not methods don’t have a `self` parameter. 

When there are *multiple* types or traits that define *non-method functions* with the same function name, Rust doesn’t always know which type you mean unless you use fully qualified syntax.

For example, lets create a trait for an animal shelter that wants to name all baby dogs `Spot`.

We make an `Animal` trait with an *associated non-method* function `baby_name`.

The `Animal` trait is implemented for the struct `Dog`, on which we also provide an *associated non-method* function `baby_name` directly.

```rs
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```

We implement the code for naming all puppies **Spot** in the baby_name associated function that is defined on `Dog`.

The `Dog` type also implements the trait `Animal`, which describes characteristics that **all** animals have.\

In main, we call the `Dog::baby_name` function, which calls the associated function defined on `Dog` directly. 

This code prints the following:

```sh
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
     Running `target/debug/traits-example`
A baby dog is called a Spot
```

This output isn’t what we wanted. 

We want to call the `baby_name` function that is part of the `Animal` trait that we implemented on `Dog` so that the code prints *A baby dog is called a puppy*.

The technique of specifying the trait name that we used earlier doesn’t help here; if we change main to the previous code, we’ll get a compilation error.

```rs
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```

Because `Animal::baby_name` doesn’t have a `self` parameter, and there could be other types that implement the `Animal` trait, Rust can’t figure out which implementation of `Animal::baby_name` we want. As such we get a compiler error.

To **disambiguate** and tell Rust that we want to use the implementation of `Animal` for `Dog` as opposed to the implementation of `Animal` for some other type, we need to use *fully qualified syntax*.

```rs
fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

We’re providing Rust with a type annotation within the *angle brackets*, which indicates we want to call the `baby_name` method from the `Animal` trait as implemented on `Dog` 

Were saying we want to treat the `Dog` type as an `Animal` for this function call.

This code will now print what we want:

```sh
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/traits-example`
A baby dog is called a puppy
```

In general, *fully qualified syntax* is defined as follows:

```rs
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

For associated functions that aren’t methods, there would not be a receiver: There would only be the list of other arguments. 

You could use fully qualified syntax everywhere that you call functions or methods.

### Using Supertraits

Sometimes you might write a *trait definition that depends on another trait*: 

For a type to implement the first trait, you want to require that type to also implement the second trait.

You would do this so that your trait definition can make use of the *associated items of the second trait*.

The trait your trait definition is relying on is called a `supertrait` of your trait.

For example, let’s say we want to make an `OutlinePrint` trait with an `outline_print` method that will print a given value formatted so that it’s framed in asterisks.

That is, given a `Point` struct that implements the standard library trait `Display` to result in `(x, y)`, when we call `outline_print` on a `Point` instance that has `1` for `x` and `3` for `y`, it should print the following:

```sh
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of the `outline_print` method, we want to use the `Display` trait’s functionality.

Therefore, we need to specify that the `OutlinePrint` trait will work only for types that also implement `Display` and provide the functionality that `OutlinePrint` needs. 

We can do that in the *trait definition* by specifying `OutlinePrint: Display`.

```rs
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we can use the `to_string` function that is automatically implemented for any type that implements Display.

If we tried to use `to_string` without adding a colon and specifying the `Display` trait after the trait name, we’d get an error saying that no method named `to_string` was found for the type `&Self` in the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that doesn’t implement `Display`, such as the `Point` struct:

```rs
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

We get an error saying that `Display` is required but not implemented:

```sh
$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:20:23
   |
20 | impl OutlinePrint for Point {}
   |                       ^^^^^ the trait `std::fmt::Display` is not implemented for `Point`
   |
note: required by a bound in `OutlinePrint`
  --> src/main.rs:3:21
   |
 3 | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`

error[E0277]: `Point` doesn't implement `std::fmt::Display`
  --> src/main.rs:24:7
   |
24 |     p.outline_print();
   |       ^^^^^^^^^^^^^ the trait `std::fmt::Display` is not implemented for `Point`
   |
note: required by a bound in `OutlinePrint::outline_print`
  --> src/main.rs:3:21
   |
 3 | trait OutlinePrint: fmt::Display {
   |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint::outline_print`
 4 |     fn outline_print(&self) {
   |        ------------- required by a bound in this associated function

For more information about this error, try `rustc --explain E0277`.
error: could not compile `traits-example` (bin "traits-example") due to 2 previous errors
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that `OutlinePrint` requires, like so:

```rs
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Then, implementing the `OutlinePrint` trait on `Point` will compile successfully, and we can call `outline_print` on a `Point` instance to display it within an outline of asterisks.

### Implementing External Traits with the Newtype Pattern

In the “Implementing a Trait on a Type” section in Chapter 10, we mentioned the **orphan rule** that states:

We’re *only* allowed to implement a trait on a type if either the trait or the type, or both, are local to our crate.

It’s possible to get around this restriction using the `newtype pattern`, which involves creating a new type in a tuple struct. 

The tuple struct will have *one field* and be a **thin wrapper** around the type for which we want to implement a trait.

Then, the **wrapper type** is local to our crate, and we can implement the trait *on the wrapper*.

*Newtype* is a term that originates from the Haskell programming language.

There is *no runtime performance penalty* for using this pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec<T>`, which the orphan rule **prevents u**s from doing directly because the `Display` trait and the `Vec<T>` type are defined *outside our crate*.

We can make a `Wrapper` struct that holds an instance of `Vec<T>`; then, we can implement `Display` on `Wrapper` and use the `Vec<T>` value:

```rs
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

The implementation of `Display` uses `self.0` to access the inner `Vec<T>` because `Wrapper` is a tuple struct and `Vec<T>` is the item at index `0` in the tuple. Then, we can use the functionality of the `Display` trait on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it *doesn’t have the methods of the value it’s holding*.

We would have to implement **all** the methods of `Vec<T>` directly on `Wrapper` such that the methods delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a `Vec<T>`.

If we wanted the new type to have every method the inner type has, implementing the `Deref` trait on the `Wrapper` to return the inner type would be a great solution.

If we didn’t want the `Wrapper` type to have **all** the methods of the inner type—for example, to restrict the `Wrapper` type’s behavior—we would have to implement *just the methods we do want* manually.

