## ## Chapter 16 - Fearless Concurrency: Transfer Data Between Threads with Message Passing

### Overview
One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data. 

To accomplish message-sending concurrency, Rust’s standard library provides an implementation of **channels**.

A **channel** is a general programming concept by which data is sent from one thread to another.

A channel has **two** halves: a **transmitter** and a **receiver**.  

One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.

A channel is said to be closed if either the transmitter **or** receiver half is dropped.

#### Channels

Let's create a channel but not do anything with it. Note that this won’t compile yet because Rust can’t tell what type of values we want to send over the channel.

```rs
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

We create a new channel using the `mpsc::channel` function, `mpsc` stands for multiple producer, single consumer. 

In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end 

The `mpsc::channel` function returns a **tuple**, the first element of which is the sending end—the transmitter—and the second element of which is the receiving end—the receiver.

Let’s move the transmitting end into a spawned thread and have it send one string so that the spawned thread is communicating with the main thread.`

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

We’re using `thread::spawn` to create a new thread and then using `move` to move `tx` into the closure so that the spawned thread **owns** tx.

The spawned thread **must** own the transmitter to be able to send messages through the channel.

We can then use reciver and the function `recv` to recieve the value sent.

```rs
let received = rx.recv().unwrap();
    println!("Got: {received}");
```

The receiver has two useful methods: `recv` and `try_recv`. We’re using `recv`, short for receive, which will **block** the main thread’s execution and wait until a value is sent down the channel.

The `try_recv` method doesn’t block, its useful if this thread has other work to do while waiting for messages.

#### Transferring Ownership Through Channels

The ownership rules play a vital role in message sending because they help you write safe, concurrent code.

Let’s do an experiment to show how channels and ownership work together to prevent problems: 

We’ll try to use a val value in the spawned thread **after** we’ve sent it down the channel. 

```rs
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Here, we try to print `val` after we’ve sent it down the channel via `tx.send`.

Once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again.

Fortunately Rust gives us an error if we try to compile the code.


#### Sending Multiple Values

Let's modify the previous example to show the concurrency in action.

```rs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

The spawned thread will now send multiple messages and pause for a second between each message.

This time, the spawned thread has a vector of strings that we want to send to the main thread. 

We iterate over them, sending each individually, and pause between each by calling the `thread::sleep` function with a `Duration` value of one second.

In the main thread, we’re not calling the `recv` function explicitly anymore: Instead, we’re treating `rx` as an **iterator**. For each value received, we’re printing it. When the channel is closed, iteration will end.

We then get the following output:

```sh
Got: hi
Got: from
Got: the
Got: thread
```

#### Creating Multiple Producers

Earlier we mentioned that `mpsc` was an acronym for multiple producer, single consumer. Let’s now put `mpsc` to use.

```rs
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // --snip--
```

This time, before we create the first spawned thread, we call `clone` on the transmitter. 

This will give us a new transmitter we can pass to the first spawned thread.

We pass the original transmitter to a second spawned thread. 

This gives us two threads, each sending different messages to the one receiver.

We will now get an ouput like this:
```rs
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

You might see the values in another order, depending on your system. This is what makes concurrency interesting as well as difficult. 