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

We want to allow the user to create a new draft blog post with `Post::new`.

Next, we want to enable a request for a review of the post, and we want `content` to return an *empty string* while waiting for the review.

When the post receives *approval*, it should get *published*, meaning the text of the post will be returned when `content` is called.

Notice that the only type we’re interacting with from the crate is the `Post` type.

This type will use the *state pattern* and will hold a value that will be one of three `state` objects representing the various `states` a post can be in.

Changing from *one state to another* will be managed **internally** within the `Post` type.

The `states` change in response to the methods called by our library’s users on the `Post` instance.

Some important things to note:

1. The methods called by the users *don’t* have to manage the state changes directly.
2. Because of how states are managed in this implementation: users can’t make a mistake with the states, such as *publishing* a `post` before it’s *reviewed*.

#### Defining Post and Creating a New Instance

Let’s get started on the implementation of the library!

We know we need a public `Post` struct that holds some content, so lets define that `struct` and its public `constructor`.

We’ll also make a private `State` trait that will define the behavior that all state objects for a `Post` must have.

Then, `Post` will hold a trait object of `Box<dyn State>` inside an `Option<T>` in a private field named `state` to hold the state object. 

You’ll see why the `Option<T>` is necessary in a bit.

```rs
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

The `State` trait defines the behavior shared by different post states.

The state objects are `Draft`, `PendingReview`, and `Published`, and they will all implement the `State` trait.

For now, the trait doesn’t have any methods, and we’ll start by defining just the `Draft` state because that is the state we want a post to start in.

When we create a new `Post`, we set its state field to a `Some` value that holds a `Box`.

This `Box` points to a new instance of the `Draft` struct.

This ensures that whenever we create a new instance of `Post`, it will start out as a draft.

Because the state field of `Post` is **private**, there is no way to create a `Post` in any other state!

In the `Post::new` function, we set the content field to a new, empty `String`.

#### Storing the Text of the Post Content

We want to be able to call a method named `add_text` and pass it a `&str` that is then added as the text content of the blog post.

We implement this as a `method`, rather than exposing the content field as `pub`, so that later we can implement a `method` that will control how the `content` field’s data is read.

Lets define this simply method:

```rs
impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

The `add_text` method takes a `mutable reference` to `self` because we’re changing the `Post` instance that we’re calling `add_text` on.

#### Ensuring That the Content of a Draft Post Is Empty

Even after we’ve called `add_text` and added some `content` to our post, we still want the `content` method to return an *empty string slice* because the post is still in the *draft state*.

For now, let’s implement the `content` method with the simplest thing that will fulfill this requirement: *always returning an empty string slice*:

```rs
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}
```

#### Requesting a Review, Which Changes the Post’s State

Next, we need to add functionality to request a review of a post, which should change its state from `Draft` to `PendingReview`:

```rs
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

We begin by giving `Post` a public method named `request_review` that will take a mutable reference to `self`.

Then, we call an internal `request_review` method on the current state of `Post`, and this second `request_review` method consumes the current state and returns a *new state*.

We add the `request_review` method to the `State` trait; all types that implement the trait will now need to implement the `request_review` method.

Note that rather than having `self`, `&self`, or `&mut self `as the first parameter of the method, we have self: `Box<Self>`.

This syntax means the method is **only valid** when called on a `Box` holding the type.

This syntax takes ownership of `Box<Self>`, **invalidating** the old state so that the state value of the `Post` can transform into a new state.

To consume the old state, the `request_review` method needs to take ownership of the state value.

This is where the `Option` in the `state field` of `Post` comes in: 

We call the take method to `take` the `Some` value out of the state field and leave a `None` in its place.

This ensures that `Post` can’t use the old state value after we’ve transformed it into a new state.

The `request_review` method on `Draft` returns a new, boxed instance of a new `PendingReview` struct.

The `PendingReview` struct also implements the `request_review` method but doesn’t do any transformations.

Rather, it returns itself because when we request a review on a post *already* in the `PendingReview` state.

The `request_review` method on `Post` is the same no matter its state value. Each state is responsible for its own rules.

#### Adding approve to Change content’s Behavior

The `approve` method will be similar to the `request_review` method: 

It will set state to the value that the current state says it should have when that state is approved:

```rs
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

We add the `approve` method to the `State` trait and add a new struct that implements `State`, the `Published` state.

Similar to the way `request_review` on `PendingReview` works, if we call the `approve` method on a `Draft`, it will have no effect because approve will return self.

When we call `approve` on `PendingReview`, it returns a new, boxed instance of the `Published` struct.

The `Published` struct implements the `State` trait, and for both the `request_review` method and the `approve` method, it returns itself because the post should stay in the `Published` state in those cases.

Now we need to update the `content` method on `Post`.

We want the value returned from `content` to depend on the current state of the `Post`.

```rs
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
```

Because the goal is to keep all of these rules *inside the structs* that implement `State`, we call a `content` method on the value in state and pass the post instance.

Then, we return the value that’s returned from using the `content` method on the state value.

We call the `as_ref` method on the `Option` because we want a reference to the value inside the `Option` rather than ownership of the value.

Because state is an `Option<Box<dyn State>>`, when we call as_`ref, an `Option<&Box<dyn State>>` is returned.

We then call the `unwrap` method, which we know will **never panic** because *we know* the methods on `Post` ensure that state will always contain a `Some` value 

At this point, when we call content on the `&Box<dyn State>`, **deref coercion** will take effect on the `&` and the `Box` so that the `content` method will ultimately be called on the type that implements the `State` trait. 

That means we need to add `content` to the `State` trait definition, and that is where we’ll put the logic for what content to return depending on which state we have.

```rs
trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

We add a **default implementation** for the content method that returns an empty string slice.

This is so we *don’t need* to implement `content` on the `Draft` and `PendingReview` structs. 

The `Published` struct will **override** the `content` method and return the value in `post.content`.

While convenient, having the `content` method on `State` determine the `content` of the `Post` is blurring the lines between the responsibility of `State` and the responsibility of `Post`.

Note: we need lifetime annotations here because were taking in a reference to `Post` and returning another reference of part of that `Post`.

And now were done! The logic related to the rules lives in the state objects rather than being scattered throughout `Post`.


#### Why not use an Enum?

An `enum` could definetly be used here and might even be more simple, however, there are some tradeoffs.

One disadvantage of using an `enum` is that every place that checks the value of the `enum` will need a `match` expression or similar to handle every possible variant. 

This could get more repetitive than this trait object solution.

#### Evaluating the State Pattern

We’ve shown that *Rust is capable* of implementing the object-oriented *state pattern* to encapsulate the different kinds of behavior a post should have in each state.

The methods on `Post` know nothing about the various behaviors.

Because of the way we organized the code, we have to look in only **one place** to know the different ways a published post can behave: 

The implementation of the `State` trait on the `Published` struct.

With the *state pattern*, the `Post` methods and the places we use `Post` don’t need match expressions, and to add a new state:

We would only need to add a new struct and implement the trait methods on that one struct in one location.

The implementation using the *state pattern* is easy to extend to add more functionality.

One **downside** of the *state pattern* is that, because the states implement the transitions between states, some of the states are *coupled to each other*.

If we add *another state* between `PendingReview` and `Published`, such as `Scheduled`, we would have to change the code in `PendingReview` to transition to `Scheduled` instead.

Another downside is that we’ve duplicated some logic.

We avoid duplication we could make default implementations of `request_review` and `approve` methods on the `State` trait that return `self`.

However, this wouldn’t work: 

When using `State` as a `trait object`, the trait doesn’t know what the concrete self will be exactly, so the return type isn’t known at **compile time**. (A dyn compatibility rule.)

Other duplication includes the similar implementations of the `request_review` and `approve` methods on `Post`.

We could consider a `macro` to deduplicate repetition.

By implementing the *state pattern* exactly as it’s defined for object-oriented languages, **we’re not taking as full advantage** of Rust’s strengths as we could.

Lets look into making some changes...

### Encoding States and Behavior as Types

Let's see how we can rethink the state pattern to get a different set of trade-offs.

Rather than **encapsulating the states and transitions completely** so that outside code has no knowledge of them, we’ll **encode the states into different types**.

Let’s consider the first part of `main`:

```rs
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

We still enable the creation of new posts in the draft state using `Post::new` and the ability to add text to the post’s `content`.

However, instead of having a `content` method on a draft post that returns an empty string, we’ll make it so that draft posts **don’t** have the `content` method at all.

That way, if we try to get a draft post’s `content`, we’ll get a compiler error telling us the *method doesn’t exist*.

As a result, it will be **impossible** for us to accidentally display draft post `content` in production because that code *won’t even compile*.

```rs
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

Both the `Post` and `DraftPost` structs have a private `content` field that stores the blog post text.

The structs *no longer have the state field* because we’re moving the encoding of the state to the types of the structs.

The `Post` struct will represent a **published post**, and it has a `content` method that returns the `content`.

We still have a `Post::new` function, but instead of returning an instance of `Post`, it returns an instance of `DraftPost`.

Because `content` is private and there aren’t any functions that return `Post`, it’s not possible to create an instance of `Post` right now.

The `DraftPost` struct has an `add_text` method, so we can add text to `content` as before, but note that `DraftPost` does not have a `content` method defined! 

To continue to enforce our constraints lets add another struct, `PendingReviewPost`.

Then lets define the `request_review` method on `DraftPost` to return a `PendingReviewPost` and defining an `approve` method on `PendingReviewPost` to return a `Post`.

```rs
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

The `request_review` and `approve` methods take ownership of `self`, thus consuming the `DraftPost` and `PendingReviewPost` instances and transforming them into a `PendingReviewPost` and a published `Post`, respectively.

This way, we won’t have any lingering `DraftPost` instances after we’ve called `request_review` on them, and so forth.

The `PendingReviewPost` struct *doesn’t have* a `content` method defined on it, so attempting to read its `content` results in a compiler error, as with `DraftPost`. 

The only way to get a published `Post` instance that does have a `content` method defined is to call the `approve` method on a `PendingReviewPost`.

The only way to get a `PendingReviewPost` is to call the `request_review` method on a `DraftPost`.

But we also have to make some small changes to `main`.

```rs
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

The changes we needed to make to `main` to reassign `post` mean that this implementation *doesn’t quite follow* the object-oriented state pattern anymore.

However, our gain is that *invalid states are now impossible* because of the type system and the type checking that happens at **compile time!**



