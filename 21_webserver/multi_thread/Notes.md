## Chapter 20 - Building a Multithreaded Web Server: From a Single-Threaded to a Multithreaded Server

### Summary

Right now, the server will process each request in turn, meaning it won’t process a second connection until the first connection is finished processing.

If the server received more and more requests, this serial execution would be less and less optimal. 

If the server receives a request that takes a long time to process, subsequent requests will have to wait until the long request is finished, even if the new requests can be processed quickly. 

We’ll need to fix this, but first we’ll look at the problem in action.

### Simulating a Slow Request

We’ll look at how a slowly processing request can affect other requests made to our current server implementation.

The example showcases handling a request to `/sleep` with a simulated slow response that will cause the server to sleep for five seconds before responding:

```rs
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // --snip--
}
```

We switched from `if` to `match` now that we have three cases.

We need to explicitly `match` on a slice of `request_line` to pattern-match against the string literal values; 

`match` **doesn’t** do automatic referencing and dereferencing, like the equality method does.

The first arm is the same as the `if` block from before. The second arm matches a request to `/sleep`. 

When that request is received, the server will sleep for five seconds before rendering the successful HTML page.

The third arm is the same as the `else` block from before.

You can see how primitive our server is: Real libraries would handle the recognition of multiple requests in a much less verbose way!

If you start the server and navigate to the `/sleep` URI and then load `/`, you’ll see that `/` waits until sleep has slept for its full five seconds before loading.

There are **multiple techniques** we could use to avoid requests backing up behind a slow request, including using async as we did Chapter 17; the one we’ll implement is a `thread pool`.

### Improving Throughput with a Thread Pool

A `thread pool` is a group of spawned threads that are **ready** and **waiting** to handle a task. 

When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task. 

The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing.

When a thread is done processing its task, it’s returned to the pool of idle threads, ready to handle a new task.

A `thread pool` allows you to process connections concurrently, increasing the throughput of your server.

We’ll limit the number of threads in the pool to a small number to protect us from DoS attacks.

Requests that come in are sent to the pool for processing. The pool will maintain a queue of incoming requests.
 
Each of the threads in the pool will pop off a request from this queue, handle the request, and then ask the queue for another request.

With this design, we can process up to `N requests` concurrently, where N is the number of threads. 

If each thread is responding to a long-running request, subsequent requests can still back up in the queue, but we’ve increased the number of long-running requests we can handle before reaching that point.

This technique is just one of many ways to improve the throughput of a web server. 

Other options you might explore are the fork/join model, the single-threaded async I/O model, and the multithreaded async I/O model. 

Before we begin implementing a thread pool, let’s talk about what using the pool should look like. 

When you’re trying to design code, writing the client interface first can help guide your design. 

Write the API of the code so that it’s structured in the way you want to call it; then, implement the functionality within that structure rather than implementing the functionality and then designing the public API.

Similar to how we used test-driven development in the project in Chapter 12, we’ll use compiler-driven development here. 

We’ll write the code that calls the functions we want, and then we’ll look at errors from the compiler to determine what we should change next to get the code to work.

Before we do that, however, we’ll explore the technique we’re not going to use as a starting point.

### Spawning a Thread for Each Request

First, let’s explore how our code might look *if it did create a new thread for every connection*.

Then, we’ll add the `thread pool` as an improvement, and contrasting the two solutions will be easier.

Below are the changes to make to `main` to spawn a new thread to handle each stream within the `for` loop:

```rs
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}
```

`thread::spawn` will create a new thread and then run the code in the `closure` in the new thread. 

If you run this code and load `/sleep` in your browser, then `/`in two more browser tabs, you’ll indeed see that the requests to `/` don’t have to wait for `/sleep` to finish.

However, as we mentioned, this will eventually *overwhelm* the system because you’d be making new threads without any limit.

### Creating a Finite Number of Threads

We want our `thread pool` to work in a similar, familiar way so that switching from `threads` to a `thread pool` doesn’t require large changes to the code that uses our API.

Below is a hypothetical interface for a `ThreadPool` struct we want to use instead of `thread::spawn`.

```rs
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

We use `ThreadPool::new` to create a new thread pool with a configurable number of threads, in this case four. 

Then, in the `for` loop, `pool.execute` has a similar interface as `thread::spawn` in that it takes a closure that the pool should run for each stream. 

We need to implement `pool.execute` so that it takes the closure and gives it to a thread in the pool to run.

This code won’t yet compile, but we’ll try so that the compiler can guide us in how to fix it.

### Building ThreadPool Using Compiler-Driven Development

Here is the first error we get:

```sh
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
  --> src/main.rs:11:16
   |
11 |     let pool = ThreadPool::new(4);
   |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

Great! This error tells us we need a `ThreadPool` type or module, so we’ll build one now. 

Our `ThreadPool` implementation will be independent of the kind of work our web server is doing. 

Let's create a `src/lib.rs` file that contains the following, which is the simplest definition of a `ThreadPool` struct that we can have for now:

```rs
pub struct ThreadPool;
```

Then, edit the `main.rs` file to bring `ThreadPool` into scope from the library crate by adding the following code to the top of `src/main.rs`:

```rs
use multi_thread::ThreadPool;
```

This code still won’t work, but let’s check it again to get the next error that we need to address:

```sh
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
  --> src/main.rs:12:28
   |
12 |     let pool = ThreadPool::new(4);
   |                            ^^^ function or associated item not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

This error indicates that next we need to create an `associated function` named `new` for `ThreadPool`.

We also know that new needs to have *one* parameter that can accept **4** as an argument and should return a `ThreadPool` instance.

Let’s implement the simplest `new` function that will have those characteristics:

```rs
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }
}
```

We chose `usize` as the type of the `size` parameter because we know that a *negative number of threads* doesn’t make any sense.

Let’s check the code again:

```sh
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
  --> src/main.rs:17:14
   |
17 |         pool.execute(|| {
   |         -----^^^^^^^ method not found in `ThreadPool`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello` (bin "hello") due to 1 previous error
```

Now the error occurs because we don’t have an `execute` method on `ThreadPool`.

Recall that we decided our thread pool should have an interface similar to `thread::spawn`.

In addition, we’ll implement the `execute` function so that it takes the `closure` it’s given and gives it to an idle thread in the `pool` to run.

We’ll define the `execute` method on `ThreadPool` to take a `closure` as a parameter.

Recall that we can take *closures as parameters* with three different traits: `Fn`, `FnMut`, and `FnOnce`.

We need to decide which kind of closure to use here.

We know we’ll end up doing something similar to the standard library `thread::spawn` implementation, so we can look at what bounds the signature of `thread::spawn` has on its parameter. 

The documentation shows us the following:

```rs
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

The `F` type parameter is the one we’re concerned with here; the `T` type parameter is related to the return value, and we’re not concerned with that.

We can see that spawn uses `FnOnce` as the trait bound on `F`.

This is probably what we want as well, because we’ll eventually pass the argument we get in `execute` to `spawn`. 

We can be further confident that `FnOnce` is the trait we want to use because the thread for running a request will only execute that request’s closure one time, which matches the `Once` in `FnOnce`.

The `F` type parameter also has the trait bound `Send` and the lifetime bound `'static`, which are useful in our situation: 

We need `Send` to transfer the closure from *one thread to another* and `'static` because we *don’t know how long* the thread will take to execute.

Let’s create an `execute` method on `ThreadPool` that will take a generic parameter of type `F` with these bounds:

```rs
impl ThreadPool {
    // --snip--
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

```

We still use the `()` after `FnOnce` because this `FnOnce` represents a closure that takes no parameters and returns the unit type `()`.

Just like function definitions, the return type can be omitted from the signature, but even if we have no parameters, we still need the parentheses.

Again, this is the simplest implementation of the execute method: It does nothing, but we’re only trying to make our code compile. 

Let’s check it again:

```sh
$ cargo check
    Checking hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
```

It compiles! But note that our library isn’t actually calling the closure passed to execute yet!

### Validating the Number of Threads in new

We aren’t doing anything with the parameters to `new` and `execute`.

Let’s implement the bodies of these functions with the behavior we want. To start, let’s think about `new`.

Earlier we chose an unsigned type for the `size` parameter because a pool with a *negative number* of threads makes no sense.

However, a pool with **zero threads** also makes no sense, yet zero is a perfectly valid `usize`.

We’ll add code to check that size is **greater than zero** before we return a `ThreadPool` instance, and we’ll have the program `panic` if it receives a zero by using the `assert!` macro:

```rs
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    // --snip--
}
```

We’ve also added some documentation for our `ThreadPool` with doc comments. 

Note that we followed good documentation practices by adding a section that calls out the situations in which our function can `panic`.

Instead of adding the `assert!` macro as we’ve done here, we could change `new` into `build` and return a `Result` like we did with `Config::build` in the I/O project.

But we’ve decided in this case that trying to create a thread pool without any threads should be an **unrecoverable error**.

### Creating Space to Store the Threads

Now that we have a way to know we have a valid number of threads to store in the pool, we can create those threads and store them in the `ThreadPool` struct before returning the struct.

But how do we *“store”* a thread? Let’s take another look at the `thread::spawn` signature:

```rs
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

The `spawn` function returns a `JoinHandle<T>`, where `T` is the type that the `closure` returns. 

Let’s try using `JoinHandle` too and see what happens.

In our case, the `closures` we’re passing to the thread pool will handle the connection and *not return anything*, so `T` will be the unit type `()`.

```rs
use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }
    // --snip--
}
```

This code will compile, but it *doesn’t create any threads yet*.

We’ve changed the definition of `ThreadPool` to hold a vector of `thread::JoinHandle<()>` instances, initialized the vector with a capacity of size, set up a `for` loop that will run some code to create the threads, and returned a `ThreadPool` instance containing them.

We’ve brought `std::thread` into scope in the library crate because we’re using `thread::JoinHandle` as the type of the items in the vector in `ThreadPool`.

Once a valid size is received, our `ThreadPool` creates a new vector that can hold size items.

The `with_capacity` function performs the same task as `Vec::new` but with an important difference: It pre-allocates space in the vector. 

Because we know we need to store `size` elements in the vector, doing this allocation up front is slightly more efficient than using `Vec::new`, which resizes itself as elements are inserted.

### Sending Code from the ThreadPool to a Thread

Here, we’ll look at how we actually create threads. 

The standard library provides `thread::spawn` as a way to create threads, and `thread::spawn` expects to get some code the thread should run as soon as the thread is created. 

However, in our case, we want to create the threads and have them wait for code that we’ll send later. 

The standard library’s implementation of threads doesn’t include any way to do that; **we have to implement it manually**.

We’ll implement this behavior by introducing a new data structure between the `ThreadPool` and the threads that will manage this new behavior.

We’ll call this data structure `Worker`, which is a common term in pooling implementations. 

The `Worker` picks up code that needs to be run and runs the code in its thread.

Think of people working in the kitchen at a restaurant: The workers wait until orders come in from customers, and then they’re responsible for taking those orders and filling them.

Instead of storing a vector of `JoinHandle<()>` instances in the thread pool, we’ll store instances of the `Worker` struct.

Each `Worker` will store a single `JoinHandle<()>` instance.

Then, we’ll implement a method on `Worker` that will take a `closure` of code to run and send it to the already running thread for execution. 

We’ll also give each `Worker` an `id` so that we can distinguish between the different instances of `Worker` in the `pool` when logging or debugging.

Here is the new process that will happen when we create a `ThreadPool`.

We’ll implement the code that sends the `closure` to the `thread` after we have `Worker` set up in this way:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`.
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a `Worker` instance that holds the `id` and a thread spawned with an empty closure.
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create a new `Worker` with that `id`, and store the `Worker` in the vector.

Lets begin:

```rs
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
    }
    // --snip--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
```

We’ve changed the name of the field on `ThreadPool` from threads to workers because it’s now holding `Worker` instances instead of `JoinHandle<()>` instances.

We use the counter in the for loop as an argument to `Worker::new`, and we store each new `Worker` in the vector named `workers`.

External code (like our server in src/main.rs) doesn’t need to know the implementation details regarding using a `Worker` struct within `ThreadPool`, so we make the `Worker` struct and its new function private.

The `Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>` instance that is created by spawning a new thread using an empty closure.

This code will compile and will store the number of `Worker` instances we specified as an argument to `ThreadPool::new`.

But we’re still not processing the `closure` that we get in execute. Let’s look at how to do that next.

### Sending Requests to Threads via Channels

