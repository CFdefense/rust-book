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

