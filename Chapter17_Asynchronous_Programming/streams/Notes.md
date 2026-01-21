## ## Chapter 17 - Asynchronous Programming: Streams: Futures in Sequence

### Overview

When we used the reciever for our async channel earlier in the chapter, the `recv` method produced a sequence of items over time.

This is an instace of a more general pattern called a `stream`.

Many concepts can naturally be represented as `streams` such as:

1. Items becoming available in a queue
2. Chunks of data being pulled incrementally from the filesystem when the full data set is too large for the computer’s memory
3. Data arriving over the network over time.

Because `streams` are **futures** we can use them with other types of futures and combine them in interesting ways.

For example, we can:

1. Batch up events to avoid triggering too many network calls
2. Set timeouts on sequences of long-running operations
3. Throttle user interface events to avoid doing needless work.

##### Iterators vs Async Recv

We saw a **sequence of numbers** previously with iterators. However, async recievers differ.

The first difference is that `iterators` are **synchronous** while the `channel reciever` is **asynchronous**.

The second difference is the **API**, when working directly with an `Iterator` we call its **synchronous** `Next` method. 

However, when we work with the `trpl::Reciever stream` we call an **asynchronous** `recv` method. 

These differing uses are similiar, which is intended as a `stream` is simply an **asynchronous form** of iteration.

Whereas the `trpl::Reciever` specifically waits to recieve messages, though, the general purpose `stream` **API** is more generic providing the `Next` item similiarly to `Iterator` but **asynchronously**.

The similarity between iterators and streams in Rust means we can actually create a `stream` from **any** `iterator`.

As with an `iterator`, we can work with a `stream` by calling its `next` method and then awaiting the output, although this wont compile yet:

```rs
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
```

We start with an array of numbers, which we convert to an iterator and then call `map` on to double all the values.

Then we convert the `iterator` into a `stream` using the `trpl::stream_from_iter` function. 

Next, we loop over the items in the `stream` as they arrive with the `while let` loop.

Unfortunately, when we try to run the code, it doesn’t compile but instead reports that there’s no `next` method available:

```sh
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

As this output explains, the reason for the compiler error is that we need the right `trait` in scope to be able to use the `next` method.

The `trait` in question is `StreamExt` not `Stream`.

The `Stream trait` defines a low-level interface that effectively combines the `Iterator` and `Future` traits. 

`StreamExt` supplies a higher-level set of APIs on top of `Stream`, including the `next` method as well as other utility methods similar to those provided by the `Iterator` trait. 

The fix to the compiler error is to add a `use statement` for `trpl::StreamExt`.

```rs
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // --snip--
```