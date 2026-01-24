## ## Chapter 17 - Asynchronous Programming: A Closer Look at the Traits for Async

### Overview

Throughout the chapter, we’ve used the `Future`, `Stream`, and `StreamExt` traits in various ways.

So far, though, we’ve avoided getting too far into the details of how they work or how they fit together.

These concepts are not totally required to be understood well in depth, however, we will review them at a deeper level alongside the `Pin` and `Unpin` traits.

### The Future Trait

Let’s start by taking a closer look at how the `Future` trait works. Here’s how Rust defines it:

```rs
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

This trait definition includes a bunch of new types and also some syntax we haven’t seen before, so let’s walk through the definition piece by piece:

First: `Future`’s **associated type** `Output` says what the future resolves to.

Second: `Future` has the `poll` method, which takes a special `Pin` reference for its `self` parameter and a **mutable reference** to a `Context` type, and returns a `Poll<Self::Output>`.

The `Poll` type returned looks like this:

```rs
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

This `Poll` type is similar to an `Option`. It has one variant that has a value, `Ready(T)`, and one that does not, `Pending`. 

The `Pending` variant indicates that the future still has work to do, so the caller will need to check again later. 

The `Ready` variant indicates that the `Future` has finished its work and the `T` value is available.

Note: When a `Poll` type becomes `Ready` one should usually **NOT** call `Poll` again after the fact, most futures will panic. This is similar to how `Iterator::next` behaves.

When you see code that uses `await`, Rust compiles it under the hood to code that calls `poll`.

Using the previous example from earlier in the chapter:

```rs
match page_title(url).await {
    Some(title) => println!("The title for {url} was {title}"),
    None => println!("{url} had no title"),
}
```

Will roughly compile to:

```rs
match page_title(url).poll() {
    Ready(page_title) => match page_title {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
    Pending => {
        // But what goes here?
    }
}
```

What should we do when the future is still `Pending`? We need some way to try again, and again, and again, until the future is finally ready. 

In other words, we need a `loop`:

```rs
let mut page_title_fut = page_title(url);
loop {
    match page_title_fut.poll() {
        Ready(value) => match page_title {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
        Pending => {
            // continue
        }
    }
}
```

If Rust compiled it to exactly that code, though, every `await` would be **blocking**, which is exactly the opposite of what we were going for!

Instead, Rust ensures that the `loop` can **hand off control** to `something` that can pause work on this future to work on other futures and then check this one again later.

That `something` is an async runtime, and this scheduling and coordination work is one of its main jobs.

Previously we also discussed awaiting values using `rx.recv`. The `recv` call returns a `future`, and awaiting the `future` polls it.

We noted that a runtime will **pause** the `future` until it’s ready with either `Some(message)` or `None` when the channel closes.

With our deeper understanding of the `Future` trait, and specifically `Future::poll`, we can see how that works.

The runtime knows the `future` isn’t ready when it returns `Poll::Pending`. 

Conversely, the runtime knows the `future` is ready and advances it when poll returns `Poll::Ready(Some(message))` or `Poll::Ready(None)`.

### The Pin Type and the Unpin Trait

Previously we saw how we could use the `trpl::join!` macro to await numerous futures.

However, it’s common to have a collection such as a `vector` containing some number futures that *won’t be known until runtime*.

Lets change our previous example where we awaited multiple futures using `trpl::join!` to instead use a vector and call `trpl::join_all` instead.

```rs
    let tx_fut = async move {
        // --snip--
    };

    let futures: Vec<Box<dyn Future<Output = ()>>> =
        vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

    trpl::join_all(futures).await;
```

We put each future into a `Box` to make them into **Trait Objects** (More on these in Chapter 18).

Using **Trait Objects** lets us treat each anonymous future produced as the same type because each of them implement the `Future` trait.

This might be surprising. After all, none of the async blocks returns anything, so each one produces a `Future<Output = ()>`.

Remember that `Future` is a trait, though, and that the compiler creates a **unique enum for each async block**, even when they have identical output types.

Just as you can’t put two different handwritten structs in a `Vec`, you can’t mix `compiler-generated enums`.

However, this doesn’t compile; here’s the relevant part of the error messages.

```sh
error[E0277]: `dyn Future<Output = ()>` cannot be unpinned
  --> src/main.rs:48:33
   |
48 |         trpl::join_all(futures).await;
   |                                 ^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
   |
   = note: consider using the `pin!` macro
           consider using `Box::pin` if you need to access the pinned value outside of the current scope
   = note: required for `Box<dyn Future<Output = ()>>` to implement `Future`
note: required by a bound in `futures_util::future::join_all::JoinAll`
  --> file:///home/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/futures-util-0.3.30/src/future/join_all.rs:29:8
   |
27 | pub struct JoinAll<F>
   |            ------- required by a bound in this struct
28 | where
29 |     F: Future,
   |        ^^^^^^ required by this bound in `JoinAll`
```

The note in this error message tells us that we should use the `pin!` macro to pin the values.

This means putting them inside the `Pin` type that guarantees the values **won’t be moved in memory**.

The error message says pinning is required because `dyn Future<Output = ()>` needs to implement the `Unpin` trait and it **currently does not**.

The `trpl::join_all` function returns a struct called `JoinAll`.

This struct is generic over a type `F`, which is constrained to implement the `Future` trait.

**Directly awaiting** a `future` with `await` pins the future implicitly. That’s why we don’t need to use `pin!` everywhere we want to await futures.

However, we’re not directly awaiting a future here. Instead, we construct a **new future**, `JoinAll`, by passing a collection of futures to the `join_all` function.

The signature for `join_all` requires that the types of the items in the collection all implement the `Future` trait, and `Box<T>` implements `Future` only if the `T` it wraps is a future that implements the `Unpin` trait.

Lets look again at the `Future` trait to further understand:

```rs
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

The `cx` parameter and its `Context` type are the key to how a `runtime` actually knows when to check any given future while still being lazy.

We dont really care about how this works in this chapter and would only be relevant if you write your own `Future` implementation.

We’ll focus instead on the type for `self`, as this is the first time we’ve seen a method where self has a **type annotation**.

A type annotation for `self` works like type annotations for other function parameters but with *two key differences*:

1. It tells Rust what type self must be for the method to be called.
2. It can’t be just any type. It’s restricted to the type on which the method is implemented, a `reference` or `smart pointer` to that type, or a `Pin` wrapping a reference to that type.

For now, it’s enough to know that if we want to `poll` a `future` to check whether it is `Pending` or `Ready(Output)`, we need a `Pin-wrapped` mutable reference to the type.

`Pin` is a wrapper for pointer-like types such as `&`, `&mut`, `Box`, and `Rc`.

`Pin` is **not** a pointer itself and doesn’t have any behavior of its own like `Rc` and `Arc` do with reference counting; it’s **purely a tool** the compiler can use to enforce constraints on pointer usage.

Recalling that `await` is implemented in terms of calls to `poll` starts to explain the error message we saw earlier, but that was in terms of `Unpin`, not `Pin`.

So how exactly does `Pin` relate to `Unpin`, and why does `Future` need `self` to be in a `Pin` type to call `poll`?

When we `move` a future—whether by pushing it into a data structure to use as an `iterator` with `join_all` or by returning it from a function **that actually means** moving the state machine Rust creates for us.

And unlike most other types in Rust, the `futures` Rust creates for `async blocks` *can end up with references to themselves* in the fields of any given variant.

By default, though, any object that has a *reference to itself* is `unsafe` to `move`, because references always point to the actual memory address of whatever they refer to.

If you move the data structure itself, those internal references will be left pointing to the old location.

Theoretically, the Rust compiler *could try* to update every reference to an object whenever it gets moved, but that could add a lot of performance overhead.

If we could instead make sure the data structure in question *doesn’t move in memory*, we wouldn’t have to update any references.

This is exactly what Rust’s borrow checker is for: *in safe code*, it prevents you from moving any item with an **active reference** to it.

`Pin` builds on that to give us the exact guarantee we need.

When we `pin` a value by wrapping a pointer to that value in `Pin`, it can **no longer move**.

Thus, if you have `Pin<Box<SomeType>>`, you actually pin the `SomeType` value, not the `Box` pointer.

In fact, the `Box` pointer can still move around freely. This is because ultimately all we care about is that the data being referenced does not move.

Most types are usually fine to move around, We only need to think about pinning when items have internal references (like these Future state machines).

Primitive values such as `numbers` and `Booleans` are safe because they obviously don’t have any internal references.

`Unpin` is a **marker trait**, similar to the `Send` and `Sync` traits we saw in Chapter 16, and thus has *no functionality of its own*.

`Unpin` informs the compiler that a given type does not need to uphold any guarantees about whether the value in question can be safely moved.

Just as with `Send` and `Sync`, the compiler implements `Unpin` automatically for all types where it can prove it is safe.

A special case is where `Unpin` is not implemented for a type. The notation for this is `impl !Unpin for SomeType`, where `SomeType` is the name of a type that does need to uphold those guarantees to be safe whenever a pointer to that type is used in a `Pin`.

In other words, there are two things to keep in mind about the relationship between `Pin` and `Unpin`:

1. `Unpin` is the “normal” case, and `!Unpin` is the special case.
2. Whether a type implements `Unpin` or `!Unpin` only matters when you’re using a pinned pointer to that type like `Pin<&mut SomeType>`.

To make that concrete, think about a `String`: it has a length and the Unicode characters that make it up.

We can wrap a `String` in `Pin`, however, String automatically implements `Unpin`, as do most other types in Rust.

As a result, we can do things that would be illegal if String implemented `!Unpin` instead, such as replacing one string with another at the exact *same location* in memory. 

This doesn’t violate the `Pin` contract, because `String` has no internal references that make it `unsafe` to move around. 

That is precisely why it implements `Unpin` rather than `!Unpin`.

Now we know enough to understand the errors reported for that `join_all` call.

We originally tried to move the futures produced by `async blocks` into a `Vec<Box<dyn Future<Output = ()>>>`, but as we’ve seen, those futures may have internal references, so they don’t automatically implement `Unpin`.

Once we `pin` them, we can pass the resulting `Pin` type into the `Vec`, confident that the underlying data in the futures will not be moved. 

Heres how we do that:

```rs
use std::pin::{Pin, pin};

// --snip--

    let tx1_fut = pin!(async move {
        // --snip--
    });

    let rx_fut = pin!(async {
        // --snip--
    });

    let tx_fut = pin!(async move {
        // --snip--
    });

    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
        vec![tx1_fut, rx_fut, tx_fut];
```

This example now compiles and runs, and we could add or remove futures from the vector at runtime and join them all.

`Pin` and `Unpin` are mostly important for building lower-level libraries, or when you’re building a runtime itself, rather than for day-to-day Rust code. 

When we see these traits in error messages, though, now we can have a better idea of how to fix the code!

### The Stream Trait

Now that you have a deeper grasp on the `Future`, `Pin`, and `Unpin` traits, we can turn our attention to the `Stream` trait.

As we learned earlier in the chapter, streams are similar to asynchronous iterators. 

Unlike `Iterator` and `Future`, however, `Stream` has *no definition in the standard library* as of this writing, but there is a very common definition from the futures crate used throughout the ecosystem.

Let’s review the definitions of the `Iterator` and `Future` traits before looking at how a `Stream` trait might merge them together.

From `Iterator`, we have the idea of a sequence: its next method provides an `Option<Self::Item>`.

From `Future`, we have the idea of readiness over time: its `poll` method provides a `Poll<Self::Output>`.

To represent a sequence of items that become ready over time, we define a `Stream` trait that puts those features together:

```rs

use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

`Stream` also defines a method to get those items. We call it `poll_next`, to make it clear that it polls in the same way `Future::poll` does and produces a sequence of items in the same way `Iterator::next` does. 

Its return type combines `Poll` with `Option`.

The outer type is `Poll`, because it has to be checked for readiness, just as a future does.

The inner type is `Option`, because it needs to signal whether there are more messages, just as an iterator does.

In the examples we saw in the “Streams: Futures in Sequence” section, though, we didn’t use `poll_next` or `Stream`, but instead used `next` and `StreamExt`. 

We could work directly in terms of the `poll_next` API by hand-writing our own `Stream` state machines, of course, just as we could work with futures directly via their `poll` method. 

Using `await` is much nicer, though, and the `StreamExt` trait supplies the `next` method so we can do just that:

```rs
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;

    // other methods...
}
```

The `StreamExt` trait is also the home of all the interesting methods available to use with streams. 

`StreamExt` is automatically implemented for every type that implements `Stream`, but these traits are defined separately to enable the community to iterate on convenience APIs without affecting the foundational trait.

In the version of `StreamExt` used in the `trpl` crate, the trait not only defines the `next` method but also supplies a default implementation of `next` that correctly handles the details of calling `Stream::poll_next`.

This means that even when you need to write your own streaming data type, you only have to implement `Stream` and then anyone who uses your data type can use `StreamExt` and its methods with it automatically.