## Chapter 10 – Traits

### Defining and Implementing Traits
- Traits define shared behavior; types implement traits to provide that behavior.

```rs
pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle { /* fields */ }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        String::from("news summary")
    }
}
```

### Trait Bounds
- Use trait bounds to restrict generic parameters.

```rs
fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
```

### Extra Notes (from chapter summary)
- A trait defines the functionality a particular type has and can share with other types.
- Different types share the same behavior if we can call the same methods on all of them.
- Trait definitions group method signatures together to define a set of required behaviors.
- A trait can have multiple methods in its body; method signatures are listed one per line and end with a semicolon.
- The compiler enforces that any type implementing a trait provides the required methods with the correct signatures.
- Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
- Traits can be used as function parameters (similar to interfaces in TypeScript).

