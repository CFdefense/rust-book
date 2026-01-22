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