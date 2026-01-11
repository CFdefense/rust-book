## ## Chapter 17 - Asynchronous Programming: Futures and the Async Syntax

### Overview

The key elements of asynchronous programming in Rust are `futures` and Rust’s `async` and `await` keywords.

#### Futures

A `future` is a value that may not be ready now but will become ready at some point in the future. (Sometimes called a `Promise` in other languages)

Rust provides a `Future` trait as a building block so that different async operations can be implemented with different data structures but with a common interface.

In Rust, futures are simply types that implement the `Future` trait. 

Each future holds its own information about the progress that has been made and what “ready” means.

#### Async

You can apply the `async` keyword to blocks and functions to specify that they can be interrupted and resumed.

Within an `async` block or `async` function, you can use the `await` keyword to `await` a future (that is, wait for it to become ready).

The process of checking with a `future` to see if its value is available yet is called `polling`.

### An Async Program

To avoid confusion of the rust ecosystem we will import and use the `trpl` crate to focus on async development.

`Tokio` is the most widely used async runtime in Rust today, especially for web applications. `trpl` will contain this crate.

Our program will be a simple webscraper. We’ll pass in two URLs from the command line, fetch both of them concurrently, and return the result of whichever one finishes first. 

#### Defining the page_title Function

Let’s start by writing a function that takes one page URL as a parameter, makes a request to it, and returns the text of the `<title>` element

```rs
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```

First, we define a function named `page_title` and mark it with the `async` keyword.

Then we use the `trpl::get` function to fetch whatever URL is passed in and add the `await` keyword to `await` the response.

Then we call the `text` function on the response to await the text of the response payload. 

We have to **explicitly** await both of these futures, because futures in Rust are **lazy**: they don’t do anything until you ask them to with the await keyword.

Once we have `response_text`, we can parse it into an instance of the Html type using `Html::parse`. 

#### What happens under the hood here

When Rust sees a **block** marked with the `async` keyword, it compiles it into a unique, anonymous data type that implements the `Future` trait.

When Rust sees a *function* marked with async, it compiles it into a `non-async` function whose body is an `async block`.

An `async` function’s **return type** is the type of the anonymous data type the compiler creates for that `async block`.

Thus, writing `async fn` is equivalent to writing a function that returns a `future` of the return type. 

The equivilant of the `page_title` function we wrote above using this equivilant would be:

```rs
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

Let’s walk through each part of the transformed version:

1. It uses the `impl Trait` syntax we discussed back in **Chapter 10** in the “Traits as Parameters” section.
2. The returned value implements the `Future` trait with an associated type of `Output`. Notice that the Output type is `Option<String>`, which is the same as the original return type from the `async fn` version of `page_title`.
3. All of the code called in the body of the original function is wrapped in an async move block.
4. This `async` block produces a value with the type `Option<String>`, as just described. That value matches the `Output` type in the return type.
5. The new function body is an async move block because of how it uses the url parameter.

#### Executing an Async Function with a Runtime

Let's add some code to our `main` function to get the title for a single page.

```rs
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    match page_title(url).await {
        Some(title) => println!("The title for {url} was {title}"),
        None => println!("{url} had no title"),
    }
}
```

Unfortunately though, this code will not compile:

```sh
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

The reason `main` can’t be marked `async` is that `async` code needs a **runtime**: a Rust crate that manages the details of executing asynchronous code.

A program’s `main` function can **initialize** a runtime, but it’s **not** a runtime itself. 

Most languages that support async bundle a runtime, but Rust does **not**. Instead, there are many different async runtimes available.

Here, and throughout the rest of this chapter, we’ll use the `block_on` function from the `trpl` crate.

Behind the scenes, calling `block_on` sets up a **runtime** using the `tokio` crate that’s used to run the `future` passed in.

Lets now use it:

```rs
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
```

#### More behind the scenes on futures

Each await point—that is, every place where the code uses the `await` keyword—represents a place where control is handed back to the `runtime`. 

To make that work, Rust needs to keep track of the **state** involved in the `async block`.

This is an invisible state machine, as if you’d written an enum like this to save the current state at each await point:

```rs
enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}
```

Fortunately, the Rust compiler creates and manages the state machine data structures for async code automatically. 

Ultimately, something has to execute this state machine, and that something is a `runtime`. 

### Racing Two URLs Against Each Other Concurrently

Now lets call `page_title` with two different URLs passed in from the command line and race them by selecting whichever future finishes first.

```rs
use trpl::{Either, Html};

fn main() {
    // collect cli arguments
    let args: Vec<String> = std::env::args().collect();

    // use block_on to initalize a runtime
    trpl::block_on(async {
        // call page title for each url
        let title_fut_1 = async { (&args[1], page_title(&args[1]).await) };
        let title_fut_2 = async { (&args[2], page_title(&args[2]).await) };

        // match the results of select
        let (url, maybe_title) = match trpl::select(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        // print who finished first
        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
```