## ## Chapter 16 - Fearless Concurrency: Using Threads to Run Code Simultaneously

### Overview
In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once. 

Within that program, you can also have independent parts that run simultaneously (threads). 

For example, a web server could have multiple threads so that it can respond to more than one request at the same time.

Adding concurrency to a program will result in faster computations at the cost of complexity.

Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run. 

This can lead to problems, such as:

1. Race conditions, in which threads are accessing data or resources in an inconsistent order.
2. Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing.
3. Bugs that only happen in certain situations and are hard to reproduce and fix reliably.

Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes careful thought and requires a code structure that is different from that in programs running in a single thread.

The Rust standard library uses a **1:1 model** of thread implementation, whereby a program uses one operating system thread per one language thread. 

#### Creating a New Thread with spawn

To create a new thread, we call the `thread::spawn` function and pass it a **closure** containing the code we want to run in the new thread. 

Example:

```rs
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Note that when the main thread of a Rust program completes, **all** spawned threads are shut down, whether or not they have finished running.

The output from this program might be a little different every time, but it will look similar to the following:

```sh
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

The calls to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run. 

The order of operation is NOT guaranteed and is determined by the operating system. 

#### Waiting for All Threads to Finish

In our previous example the spawned thread stopped prematurely most of the time due to the main thread ending.

In addition there is no guarantee on the order in which threads run, so we also can’t guarantee that the spawned thread will get to run at all.

We can fix the problem of the spawned thread not running or of it ending prematurely by saving the return value of `thread::spawn` in a variable.

The return type of `thread::spawn` is `JoinHandle<T>`.

A `JoinHandle<T>` is an ***owned*** value that, when we call the `join` method on it, will wait for its thread to finish. 

Heres an example of `join` in action such that we wait for it to complete before main finishes:

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates. 

**Blocking** a thread means that thread is prevented from performing work or exiting.

The output will now be:

```sh
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

The two threads continue alternating, but the main thread now waits because of the call to `handle.join()` and does not end until the spawned thread is finished.

But let’s see what happens when we instead move `handle.join()` before the for loop in main, like this:

```rs
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

We observe the following now:

```sh
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

#### Using move Closures with Threads

We’ll often use the `move` keyword with **closures** passed to `thread::spawn` because the closure will then TAKE ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another. 

We talked about `move` before in **Chapter 13** in the context of just closures, now we’ll concentrate more on the interaction between `move` and `thread::spawn`.

Consider the following example:

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

In the example we spawn a thread and attempt to use the vector **v**, however, this does not compile.

Rust will attempt to infer how to capture v, and because println! only needs a reference to v, the closure tries to borrow v. 

However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know whether the reference to v will always be valid.

Heres why the lifetime matters:

```rs
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

Say the spawned thread was placed in the background, and in that time main continued and dropped v. When the spawned thread executes v is no longer valid. Ugh oh!

**Instead**: by adding the `move` keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values. 

```rs
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```