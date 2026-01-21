## ## Chapter 17 - Asynchronous Programming: Applying Concurrency with Async

### Overview

In this section, we’ll apply async to some of the same concurrency challenges we tackled with threads in Chapter 16.

In many cases, the APIs for working with concurrency using async are very similar to those for using threads. 

In other cases, they end up being quite different. 

#### Creating a New Task with spawn_task

Similiarly to our first task in Chapter 16, lets see the equivilant of counting using two threads but with `async`.

```rs
use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

As our starting point, we set up our main function with `trpl::block_on` so that our top-level function can be async.

This code behaves similarly to the **thread-based** implementation—including the fact that you may see the messages appear in a different order in your own terminal when you run it:

```sh
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

**With threads**: If you want it to run all the way to the task’s completion, you will need to use a join handle to wait for the first task to complete.
**With Async**: We can use await to do the same thing, because the task handle itself is a future.

```rs
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
```

This updated version runs until both loops finish:

```sh
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

So far, it looks like `async` and `threads` give us similar outcomes, just with different syntax: using `await` instead of calling `join` on the join handle, and awaiting the `sleep` calls.

The bigger difference is that we didn’t need to spawn another operating system thread to do this.

Previously we saw how to use the `join` method on the `JoinHandle` type returned when you call`std::thread::spawn`. The `trpl::join` function is similar, but for futures.

When you give it **two** futures, it produces a **single new future** whose output is a tuple containing the output of each future you passed in once they both complete.

```rs
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
```

When we run this, we see both futures run to completion:

```sh
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

Now, you’ll see the exact same order every time, which is very different from what we saw with threads and with `trpl::spawn_task`.

That is because the `trpl::join` function is **fair**, meaning it checks each future equally often, alternating between them, and never lets one race ahead if the other is ready.

**With threads**: the operating system decides which thread to check and how long to let it run. 
**With async Rust**: the runtime decides which task to check. 

#### Sending Data Between Two Tasks Using Message Passing

Sharing data between futures will also be familiar: we’ll use message passing again, but this time with **async versions** of the types and functions.

```rs
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
```

Here, we use `trpl::channel`, an **async version** of the multiple-producer, single-consumer channel API we used with threads back in Chapter 16.

Notice that we don’t have to spawn a separate thread or even a task; we merely need to await the `rx.recv` call.

The synchronous `Receiver::recv` method in `std::mpsc::channel` blocks until it receives a message.

The `trpl::Receiver::recv` method does not, because it is `async`. Instead of blocking, it hands control back to the runtime until either a message is received or the send side of the channel closes.

Lets make this asychronous now by sending a series of messages and sleeping in between them:

```rs
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
```

We use the `while let` conditional loop here. This is the loop version of the `if let` construct. The loop will continue executing as long as the pattern it specifies continues to match the value.

If the result of calling `rx.recv().await` is `Some(message)`, we get access to the message and we can use it in the loop body, just as we could with `if let`. 

If the result is `None`, the loop ends. Every time the loop completes, it hits the `await` point again, so the runtime pauses it again until another message arrives.

Unfortunately, there are still a couple of problems:

1. The messages do not arrive at half-second intervals. They arrive all at once, 2 seconds (2,000 milliseconds) after we start the program. 
2. This program also never exits! Instead, it waits forever for new messages. You will need to shut it down using ctrl-C.

#### Code Within One Async Block Executes Linearly

Let’s start by examining why the messages come in all at once after the full delay, rather than coming in with delays between each one.

Within a given `async block`, the order in which `await` keywords appear in the code is **also** the order in which they’re executed when the program runs.

There’s only **one** `async block` in our code, so everything in it runs **linearly**. There’s still no concurrency. 

All the `tx.send` calls happen, interspersed with all of the `trpl::sleep` calls and their associated `await` points. Only then does the `while let` loop get to go through any of the await points on the recv calls.

To get the behavior we want, where the sleep delay happens between each message, we need to put the `tx` and `rx` operations in their own `async blocks`:

```rs
        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
```

The messages now get printed at 500-millisecond intervals, rather than all in a rush after 2 seconds.

#### Moving Ownership Into an Async Block

The program still never exits, though, because of the way the while let loop interacts with `trpl::join`. 

Right now, the `async block` where we send the messages only borrows `tx` because sending a message doesn’t require ownership, but if we could `move` `tx` into that `async block`, it would be dropped once that block ends.
`
The move dynamics apply to `async blocks`, so the `move` keyword works with `async blocks` just as it does with closures.

```rs
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            // --snip--
```

When we run this version of the code, it shuts down gracefully after the last message is sent and received. 

Next, let’s see what would need to change to send data from more than one future.

#### Joining a Number of Futures with the join! Macro

This `async channel` is also a multiple-producer channel, so we can call `clone` on `tx` if we want to send messages from **multiple** futures.

```rs
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
```

First, we clone `tx`, creating `tx1` outside the first async block. 

We move `tx1` into that block just as we did before with `tx`. 

Then, later, we `move` the original `tx` into a new async block, where we send more messages on a slightly slower delay.

Both of the `async blocks` for sending messages need to be `async move blocks` so that both `tx` and `tx1` get **dropped** when those blocks finish.

Otherwise, we’ll end up back in the same infinite loop we started out in.

Finally, we switch from `trpl::join` to `trpl::join!` to handle the additional future: the `join!` macro awaits an arbitrary number of futures where we know the number of futures at compile time.

