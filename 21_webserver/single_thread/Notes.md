## Chapter 20 - Final Project: Building a Multithreaded Web Server: Building a Single-Threaded Web Server

### Summary

We’ll start by getting a single-threaded web server working.

The two main protocols involved in web servers are _Hypertext Transfer Protocol_ (HTTP) and _Transmission Control Protocol_ (TCP).

Both protocols are **request-response protocols**, meaning a client initiates **requests** and a server listens to the requests and provides a **response** to the client.

**TCP** is the lower-level protocol that describes the details of how information gets from one server to another but _doesn’t specify_ what that information is.

**HTTP** _builds on top_ of **TCP** by defining the contents of the requests and responses.

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

A single stream represents an _open connection_ between the **client** and the **server**.

We will read from the `TcpStream` to see what the client sent and then write our response to the stream to send data back to the client.

Overall, this `for` loop will process each connection in turn and produce a series of streams for us to handle.

For now, our handling of the stream consists of calling `unwrap` to terminate our program if the stream has any errors; if there aren’t any errors, the program prints a message.

The reason we might **receive errors** from the `incoming` method when a client connects to the server is that _we’re not actually_ iterating over connections.

Instead, we’re iterating over **connection attempts**.

The connection might not be successful for a number of reasons, many of them operating system specific.

When we run the program and open the connection in our web browser, you’ll see multiple messages printed for one browser request.

The reason might be that the browser is making a request for the page as well as a request for **other resources**, like the favicon.ico icon that appears in the browser tab.

It could also be that the browser is trying to connect to the server multiple times because the server isn’t responding with any data.

When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the `drop` implementation.

Browsers sometimes deal with closed connections by retrying, because the problem might be temporary.

The important factor is that we’ve successfully gotten a handle to a **TCP** connection!

### Reading the Request

Let’s implement the functionality to _read_ the request from the browser!

To separate the concerns of first getting a connection and then taking some action with the connection, we’ll start a _new function_ for processing connections.

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

The browser signals the _end_ of an HTTP request by sending _two newline characters in a row_, so to get one request from the stream, we take lines until we get a line that is the empty string.

Depending on your browser, you might get slightly different output.

Now that we’re printing the request data, we can see why we get multiple connections from one browser request by looking at the path after GET in the first line of the request.

If the repeated connections are all requesting `/`, we know the browser is trying to fetch `/` repeatedly because it’s not getting a response from our program.

Example Request From Running Our Program:

```sh
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
```

### Looking More Closely at an HTTP Request

HTTP is a text-based protocol, and a request takes this format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

The first line is the request line that holds information about what the client is requesting.

The first part of the request line indicates the `method` being used, such as `GET` or `POST`, which describes **how** the client is making this request.

Our client used a `GET` request, which means it is asking for information.

The next part of the request line is `/,` which indicates the `uniform resource identifier` (URI) the client is requesting:

A `URI` is almost, but not quite, the same as a `uniform resource locator` (URL).

We can just mentally substitute `URL` for `URI` here.

The last part is the `HTTP version` the client uses, and then the request line ends in a `CRLF` sequence.

`CRLF` stands for `carriage return and line feed`, which are terms from the typewriter days!

The `CRLF` sequence can also be written as `\r\n`, where `\r` is a carriage return and `\n` is a line feed.

The `CRLF` sequence **separates** the request line from the rest of the request data.

Note that when the `CRLF` is printed, we see a **new line** start rather than `\r\n`.

Looking at the request line data we received from running our program so far:

We see that `GET` is the **method**, `/` is the request **URI**, and `HTTP/1.1` is the **version**.

After the request line, the remaining lines starting from Host: onward are headers. GET requests have no body.

Now that we know what the browser is asking for, let’s send back some data!

### Writing a Response

We’re going to implement sending data in response to a client request. Responses have the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

The first line is a **status line** that contains the `HTTP version` used in the response, a `numeric status code` that summarizes the result of the request, and a `reason phrase` that provides a text description of the status code.

After the `CRLF` sequence are any **headers**, another `CRLF` sequence, and the **body** of the response.

Here is an example response that uses HTTP version 1.1 and has a status code of 200, an OK reason phrase, no headers, and no body:

```
HTTP/1.1 200 OK\r\n\r\n
```

Let’s write this to the stream as our response to a successful request, well handle this in handle_connection:

```rs
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
```

The first new line defines the response variable that holds the success message’s data.

Then, we call `as_bytes` on our response to convert the string data to bytes.

The `write_all` method on stream takes a `&[u8]` and sends those bytes directly down the connection.

Because the `write_all` operation could fail, we use `unwrap` on any error result as before.

### Returning Real HTML

Let’s implement the functionality for returning more than a blank page.

We'll create a new file `hello.html` in the root of the project directory:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

This is a minimal HTML5 document with a heading and some text.

To return this from the server when a request is received, we’ll modify `handle_connection`:

```rs
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

We’ve added `fs` to the use statement to bring the standard library’s filesystem module into scope.

Next, we use `format!` to add the file’s contents as the body of the success response.

To ensure a valid HTTP response, we add the Content-Length header, which is set to the size of our response body—in this case, the size of `hello.html`.

When we run this code and load `127.0.0.1:7878` in our browser; we will see the HTML rendered!

Currently, we’re **ignoring** the request data in `http_request` and just sending back the contents of the HTML file unconditionally.

That means if you try requesting `127.0.0.1:7878/something-else` in your browser, you’ll still get back this same HTML response.

At the moment, our server is very limited and does not do what most web servers do.

We want to customize our responses depending on the request and only send back the HTML file for a well-formed request to `/`.

### Validating the Request and Selectively Responding

Right now, our web server will return the HTML in the file no matter what the client requested.

Let’s add functionality to check that the browser is requesting `/` before returning the HTML file and to return an error if the browser requests anything else.

For this we need to modify `handle_connection`:

```rs
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}
```

This new code _checks the content_ of the request received against what we know a request for `/` looks like and adds if and else blocks to treat requests differently.

We’re only going to be looking at the _first line of the HTTP request_, so rather than reading the entire request into a vector, we’re calling `next` to get the **first item** from the `iterator`.

The first `unwrap` takes care of the `Option` and stops the program if the `iterator` has _no items_.

The second `unwrap` handles the `Result` and has the same effect as the `unwrap` that was in the `map`.

Next, we check the `request_line` to see if it equals the request line of a GET request to the `/` path. If it does, the if block returns the contents of our HTML file.

If the `request_line` does not equal the GET request to the `/` path, it means we’ve received some other request. We’ll add code to the else block in a moment to respond to all other requests.

When we run this code now and request `127.0.0.1:7878`; we should get the HTML in `hello.html`.

However, if we make any other request, such as `127.0.0.1:7878/something-else`, we'll get a connection error.

Lets add an else block to return a response with the _status code 404_, which signals that the content for the request was not found:

```rs
// --snip--
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
```

Here, our response has a status line with _status code 404_ and the reason phrase NOT FOUND.

The body of the response will be the HTML in the file `404.html`.

We'll need to create a `404.html` file next to `hello.html` for the error page:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
```

### Refactoring

At the moment, the `if` and `else` blocks have a lot of _repetition_: They’re both reading files and writing the contents of the files to the stream.

The only differences are the status line and the filename.

Let’s make the code more concise by pulling out those differences into separate if and else lines that will assign the values of the status line and the filename to variables.

We can then use those variables unconditionally in the code to read the file and write the response.

```rs
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
```

Now the `if` and `else` blocks only return the appropriate values for the status line and filename in a tuple; we then use destructuring to assign these two values to `status_line` and `filename` using a pattern in the `let` statement.

The _previously duplicated code_ is now outside the `if` and `else` blocks and uses the `status_line` and `filename` variables.

This makes it easier to see the **difference** between the two cases, and it means we have only one place to update the code if we want to change how the file reading and response writing work.

Currently, our server runs in a _single thread_, meaning it can only serve **one request at a time**.

Let’s examine how that can be a problem by simulating some slow requests.

Then, we’ll fix it so that our server can handle multiple requests at once.
