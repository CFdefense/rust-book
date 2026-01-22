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

