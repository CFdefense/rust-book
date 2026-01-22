## ## Chapter 16 - Fearless Concurrency: Shared-State Concurrency

### Overview

Message passing via **channels** is a fine way to handle concurrency, but it’s not the only way.

Another method would be for multiple threads to access the same shared data.

What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts (Go) caution not to use memory sharing?

Shared-memory concurrency is like multiple ownership: **Multiple** threads can access the same memory location at the same time.

#### Controlling Access with Mutexes

Mutex is an abbreviation for **mutual exclusion**, as in a mutex allows only one thread to access some data at any given time.

To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s **lock**. 

The **lock** is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. 

Mutexes have a reputation for being difficult to use because you have to remember two rules:

1. You must attempt to acquire the lock before using the data.
2. When you’re done with the data that the mutex guards, you must unlock the data so that other threads can acquire the lock.

Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels. 

However, thanks to Rust’s type system and ownership rules, you **can’t** get locking and unlocking wrong.

#### The API of Mutex<T>

As an example of how to use a mutex, let’s start by using a mutex in a single-threaded context:

```rs
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

To access the data inside the mutex, we use the `lock` method to acquire the lock.

The call to lock would fail if another thread holding the lock panicked. 

In that case, no one would ever be able to get the lock, so we’ve chosen to `unwrap` and have this thread `panic` if we’re in that situation.

After we’ve acquired the lock, we can treat the return value, named num in this case, as a `mutable reference` to the data inside.

The call to `lock` returns a type called `MutexGuard`, wrapped in a `LockResult` that we handled with the call to `unwrap`. 

The `MutexGuard` type implements `Deref` to point at our inner data; the type also has a `Drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope.

#### Shared Access to Mutex<T>

Now let’s try to share a value between multiple threads using `Mutex<T>`.

We’ll spin up **10** threads and have them each increment a counter value by 1, so the counter goes from 0 to 10.

```rs
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

We create a counter variable to hold an i32 inside a `Mutex<T>`.

We use thread::spawn and give all the threads the same closure that: 

1. Moves the counter into the thread.
2. Acquires a lock on the Mutex<T> by calling the lock method
3. Adds 1 to the value in the mutex.

When a thread finishes running its closure, num will go out of scope and release the lock so that another thread can acquire it.

In the main thread, we collect all the join handles. We call join on each handle to make sure all the threads finish. 

However, this code will not compile:

The error message states that the **counter** value was moved in the previous iteration of the loop. 

Rust is telling us that we **can’t** move the ownership of lock counter into multiple threads.

#### Multiple Ownership with Multiple Threads

In Chapter 15, we gave a value to multiple owners by using the smart pointer `Rc<T>` to create a reference-counted value.

Let’s do the same here and see what happens. 

We’ll wrap the `Mutex<T>` in `Rc<T>`.

```rs
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

This however will still result in a compiler error.

The important part to focus on: **`Rc<Mutex<i32>>` cannot be sent between threads safely**. 

The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>`.

Unfortunately, `Rc<T>` is not safe to share across threads.

#### Atomic Reference Counting with Arc<T>

Fortunately, `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations. 

The a stands for **atomic**, meaning it’s an atomically reference-counted type. 

Atomics work like primitive types but are safe to share across threads.

Let's now use `Arc<T>` instead of `Rc<T>`:

```rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

This code will print the following:

```sh
Result: 10
```

#### Comparing RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

You might have noticed that counter is **immutable** but that we could get a **mutable** reference to the value inside it.

This means `Mutex<T>` provides interior mutability, as the `Cell` family does.

Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>`.

Recall from Chapter 15 that using `Rc<T>` came with the risk of creating **reference cycles**, where two `Rc<T> `values refer to each other, causing memory leaks.

Similarly, `Mutex<T>` comes with the risk of creating deadlocks.

