## Chapter 15 – Smart Pointers: Treating Smart Pointers Like Regular References

### Overview
Implementing the Deref trait allows you to customize the behavior of the dereference operator *.

By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

#### Following The Reference to the Value
A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.

```rs
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
In the above example x is 5 and y is a reference to x. We can assert x is 5 but to make an assertion about the value in y we must dereference it.

#### Using Box<T> Like a Reference
We can rewrite the previous example to use Box<T> instead of a reference:

```rs
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

The main difference here is that here we set y to be an instance of a box pointing to a copied value of x rather than a reference pointing to the value of x.

In the last assertion, we can use the dereference operator to follow the box’s pointer in the same way that we did when y was a reference. 

#### Defining Our Own Smart Pointer
Let’s build a wrapper type similar to the Box<T> type provided by the standard library to experience how smart pointer types behave differently from references by default. Then, we’ll look at how to add the ability to use the dereference operator.

Well begin by copying how Box<T> works under the hood and our previous test.
```rs
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Unfortunately we Error: Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type. To enable dereferencing with the * operator, we implement the Deref trait.

#### Implementing the Deref Trait

The Deref trait, provided by the standard library, requires us to implement one method named `deref` that borrows self and returns a reference to the inner data. Lets implement Deref to add to the definition of MyBox<T>.

```rs
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter, but you don’t need to worry about them for now

Without the Deref trait, the compiler can only dereference `&` references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a reference that it knows how to dereference.

When we call *y rust translates it to:
```rs
*(y.deref())
```
if it implements Deref. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.

#### Using Deref Coercion in Functions and Methods

Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. 

For example, `deref coercion` can convert `&String` to `&str` because String implements the Deref trait such that it returns &str.

Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *.

To see deref coercion in action, let’s use the MyBox<T> type as well as the implementation of Deref, in combination with the definition of a function that has a string slice parameter.

```rs
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {}
```

Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>
```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Without Deref coersion we would need to call `hello` like this:
```rs
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

The (*m) dereferences the MyBox<String> into a String. Then, the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.

This code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.

When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

#### Handling Deref Coercion with Mutable References
Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

- From &T to &U when T: Deref<Target=U>
- From &mut T to &mut U when T: DerefMut<Target=U>
- From &mut T to &U when T: Deref<Target=U>

The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.

The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: Immutable references will never coerce to mutable references. 