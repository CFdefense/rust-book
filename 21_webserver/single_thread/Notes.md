## Chapter 20 - Final Project: Building a Multithreaded Web Server: Building a Single-Threaded Web Server

### Summary

We’ll start by getting a single-threaded web server working.

The two main protocols involved in web servers are *Hypertext Transfer Protocol* (HTTP) and *Transmission Control Protocol* (TCP). 

Both protocols are **request-response protocols**, meaning a client initiates **requests** and a server listens to the requests and provides a **response** to the client.

**TCP** is the lower-level protocol that describes the details of how information gets from one server to another but *doesn’t specify* what that information is.

**HTTP** *builds on top* of **TCP** by defining the contents of the requests and responses.

### Listening to the TCP Connection

Our web server needs to listen to a TCP connection, so that’s the first part we’ll work on.

The standard library offers a `std::net` module that lets us do this. 

Lets create a project that listens to the local address `127.0.0.1:7878` for incoming **TCP** streams.

```rs
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
```

Using `TcpListener`, we can listen for TCP connections at the address `127.0.0.1:7878`.

We’ve chosen port `7878` for two reasons: 

1. HTTP isn’t normally accepted on this port, so our server is unlikely to conflict with any other web server you might have running on your machine.
2. 7878 is rust typed on a telephone. lol

The `bind` function in this scenario works like the `new` function in that it will return a new `TcpListener` instance.

The function is called `bind` because, in networking, connecting to a port to listen to is known as “binding to a port.”

The `bind` function returns a `Result<T, E>`, which indicates that it’s possible for binding to **fail**, for example, if we ran two instances of our program and so had two programs listening to the same port.

Because we’re writing a basic server just for learning purposes, we won’t worry about handling these kinds of errors; instead, we use `unwrap` to stop the program if errors happen.

The `incoming` method on `TcpListener` returns an **iterator** that gives us a sequence of streams (more specifically, streams of type `TcpStream`).

A single stream represents an *open connection* between the **client** and the **server**.

We will read from the `TcpStream` to see what the client sent and then write our response to the stream to send data back to the client.

Overall, this `for` loop will process each connection in turn and produce a series of streams for us to handle.

For now, our handling of the stream consists of calling `unwrap` to terminate our program if the stream has any errors; if there aren’t any errors, the program prints a message.

The reason we might **receive errors** from the `incoming` method when a client connects to the server is that *we’re not actually* iterating over connections. 

Instead, we’re iterating over **connection attempts**.

The connection might not be successful for a number of reasons, many of them operating system specific.

When we run the program and open the connection in our web browser, you’ll see multiple messages printed for one browser request.

The reason might be that the browser is making a request for the page as well as a request for **other resources**, like the favicon.ico icon that appears in the browser tab.

It could also be that the browser is trying to connect to the server multiple times because the server isn’t responding with any data.

When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation. 

Browsers sometimes deal with closed connections by retrying, because the problem might be temporary.

The important factor is that we’ve successfully gotten a handle to a **TCP** connection!

### Reading the Request

Let’s implement the functionality to *read* the request from the browser!

To separate the concerns of first getting a connection and then taking some action with the connection, we’ll start a *new function* for processing connections.

In this new `handle_connection` function, we’ll read data from the **TCP** stream and print it so that we can see the data being sent from the browser.

```rs
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
```

We bring `std::io::BufReader` and `std::io::prelude` into scope to get access to traits and types that let us read from and write to the stream.

In the `for` loop in the `main` function, instead of printing a message that says we made a connection, we now call the new `handle_connection` function and pass the `stream` to it.

In the `handle_connection` function, we create a new `BufReader` instance that wraps a reference to the `stream`.

The `BufReader` adds buffering by managing calls to the `std::io::Read` trait methods for us.

We create a variable named `http_request` to collect the lines of the request the browser sends to our server. 

We indicate that we want to collect these lines in a vector by adding the `Vec<_>` type annotation.

`BufReader` implements the `std::io::BufRead` trait, which provides the `lines` method.

The `lines` method returns an iterator of `Result<String, std::io::Error>` by splitting the `stream` of data whenever it sees a newline byte.

To get each `String`, we `map` and `unwrap` each `Result`.

The `Result` might be an error if the data isn’t valid UTF-8 or if there was a problem reading from the `stream`.

Again, a production program should handle these errors more gracefully, but we’re choosing to stop the program in the error case for simplicity.

The browser signals the *end* of an HTTP request by sending *two newline characters in a row*, so to get one request from the stream, we take lines until we get a line that is the empty string.

Depending on your browser, you might get slightly different output.

Now that we’re printing the request data, we can see why we get multiple connections from one browser request by looking at the path after GET in the first line of the request.

If the repeated connections are all requesting `/`, we know the browser is trying to fetch `/` repeatedly because it’s not getting a response from our program.

### Looking More Closely at an HTTP Request