## Chapter 18 - Object-Oriented Programming Features: Implementing an Object-Oriented Design Pattern

### Overview

The *state pattern* is an object-oriented design pattern.

The crux of the pattern is that we define a *set of states* a value can have internally.

The states are represented by a set of **state objects**, and the value’s behavior changes based on its **state**.

We’re going to use an example of a **blog post** struct that has a field to hold its **state**, which will be a **state object** from the set “draft,” “review,” or “published.”

The advantage of using the *state pattern* is that, when the business requirements of the program change:

We **won’t** need to change the code of the value holding the state or the code that uses the value. 

We’ll **only** need to update the code inside one of the **state objects** to change its rules or perhaps add more **state objects**.

First, we’re going to implement the *state pattern* in a more traditional object-oriented way.

Then, we’ll use an approach that’s a bit more natural in **Rust**.

The final functionality will look like this:

1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print so that unapproved posts can’t accidentally be published.

Any other changes attempted on a post should have no effect.

For example, if we try to approve a draft blog post *before* we’ve requested a review, the post should **remain** an unpublished draft.

### Attempting Traditional Object-Oriented Style

This section’s implementation is more of a *traditional* object-oriented style, which is possible to write in Rust, but doesn’t take advantage of some of Rust’s strengths.

Later, we’ll demonstrate a different solution that still uses the object-oriented design pattern but is structured in a way that might look less familiar to programmers with object-oriented experience. 

We’ll *compare* the two solutions to experience the trade-offs of designing Rust code differently than code in other languages.

This is an example usage of the `API` we’ll implement in a library crate named `blog`. This won’t compile yet because we haven’t implemented the `blog` crate.

```rs
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

We want to allow the user to create a new draft blog post with Post::new.