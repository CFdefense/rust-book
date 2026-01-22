use std::fmt::{Display, Debug};
// For example, let’s say we have multiple structs that hold various kinds and amounts of text
// We want to make a media aggregator library crate named aggregator that 
// Can display summaries of data that might be stored in a NewsArticle or SocialPost instance
// Well declare this trait summary which has the method summarize
// The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
// We can also use a default implementation by adding the logic instead of a ';'
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation
    fn summarize_author(&self) -> String;
}

// now that weve defined the trait we can go and implement the trait
// lets begin by defining our two structs NewsArticle and SocialPost
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

// we then can implement the trait for the structs and define struct specific functions that fit the summarize signature defined in the trait
impl Summary for NewsArticle {
    // leave blank to implement the default trait method logic
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// here well define a function which accepts an item: anything which implements trait Summary
// this syntax is valid and works well if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary)
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
}

// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound
// this below function signature is equivalent to the above one using impl
// If we want to force both parameters to have the same type, however, we must use a trait bound
pub fn trait_bound_notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// we can also specify we want only a type which implements multiple traits
pub fn notify_multi_type(item: &(impl Summary + Display)) {}

// this is also valid with trait-bound syntax
pub fn trait_bound_notify_multi_type<T: Summary + Display>(item: &T) {}

// trait bound sytnax can get confusing so rust has another simplier syntax style
// instead of the following:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {1}
// you can use the following syntax:
fn some_function_better<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{1}

// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.
// By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait 
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// we can optionally and conditionally implement methods using traits
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// here we only implement the method cmp_display if the type also implements traits Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait. 
// Implementations of a trait on any type that satisfies the trait bounds are called 'blanket implementations'
// For example, the standard library implements the ToString trait on any type that implements the Display trait.
// impl<T: Display> ToString for T {
//      // --snip--
// }
// Because the standard library has this blanket implementation, 
// we can call the to_string method defined by the ToString trait on any type that implements the Display trait.

pub fn main() {
    // now users can use the trait method on types that implement it
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    // lets use the default method implementaton on NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // call the trait method
    println!("1 new social post: {}", post.summarize());

    // now calling the default implementation of summarize will call the summarize_author automatically
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new social post: {}", post.summarize());

}