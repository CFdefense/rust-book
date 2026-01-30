## ## Chapter 17 - Asynchronous Programming: Putting It All Together: Futures, Tasks, and Threads

### Overview

As we saw in Chapter 16, one approach to concurrency is through **Threads**.

However, weve seen a new approach in this chapter using `async` with `futures` and `streams`.

So when do we use `threads` and when do we use `async`, the answer is *it depends*.

Sometimes the best approach is using them both together!

`Threads` have some downsides such as memory usage and requiring hardware and an `OS` that supports them.

`Async` uses tasks to complete its work, a `task` is similiar to a thread but instead of being managed by the `OS` its managed by the `runtime`.

There’s a reason the APIs for spawning `threads` and spawning `tasks` are so similar.

`Threads` act as a boundary for sets of synchronous operations; concurrency is possible *between* threads.

`Tasks` act as a boundary for sets of asynchronous operations; concurrency is possible both *between* and *within* tasks, because a task can *switch* between futures in its body.

The `runtime`, specifically, its `executor`, manages tasks, and tasks manage futures.

This **doesn’t mean** that `async` tasks are always better than `threads` (or vice versa).

And it turns out that `threads` and `tasks` often *work very well together*, because tasks can (at least in some runtimes) be moved around between threads.

Many `runtimes` use an approach called `work stealing` to transparently move tasks around between threads, based on how the threads are currently being utilized.

When thinking about which method to use when, consider these rules of thumb:

1. If the work is very *parallelizable* (that is, CPU-bound), such as processing a bunch of data where each part can be processed separately, **threads** are a better choice.
2. If the work is very *concurrent* (that is, I/O-bound), such as handling messages from a bunch of different sources that may come in at different intervals or different rates, **async** is a better choice.

And if you need both **parallelism** and **concurrency**, you don’t have to choose between `threads` and `async`. 

You can use them together freely, letting each play the part it’s best at. For example:

```rs
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

We begin by creating an `async channel`, then spawning a `thread` that takes `ownership` of the sender side of the channel using the `move` keyword.

Within the `thread`, we send the numbers 1 through 10, sleeping for a second between each. 

Finally, we run a `future` created with an `async block` passed to `trpl::block_on` just as we have throughout the chapter.

In that `future`, we `await` those messages, just as in the other message-passing examples we have seen.

