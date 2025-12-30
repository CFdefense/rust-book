## Chapter 14 – More About Cargo and Crates.io

### Publishing a Crate to Crates.io

We’ve used packages from crates.io as dependencies of our project, but you can also share your code with other people by publishing your own packages.

### Making Useful Documentation Comments

Using documentation comments `///` will generate HTML documentation for your functions

Example:
```rs
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

#### Generating HTML Docs

We can then generate the HTML for this using `cargo doc`. The generated HTML is then available in `target/doc`

For convenience, running `cargo doc --open` will build the HTML for your current crate’s documentation (as well as the documentation for all of your crate’s dependencies) and open the result in a web browser.

**You can use Markdown syntax in your `///` document comments.**

#### What Should You Documents?

Some common sections to include in your comments:
- Panics: These are the scenarios in which the function being documented could panic. Callers of the function who don’t want their programs to panic should make sure they don’t call the function in these situations.

- Errors: If the function returns a Result, describing the kinds of errors that might occur and what conditions might cause those errors to be returned can be helpful to callers so that they can write code to handle the different kinds of errors in different ways.

- Safety: If the function is unsafe to call (we discuss unsafety in Chapter 20), there should be a section explaining why the function is unsafe and covering the invariants that the function expects callers to uphold.

Most documentation comments don’t need all of these sections, but this is a good checklist to remind you of the aspects of your code users will be interested in knowing about.

#### Documentation Comments as Tests

Adding example code blocks in your documentation comments can help demonstrate how to use your library and has an additional bonus: **Running cargo test will run the code examples in your documentation as tests!**

This is really cool because it ensures the documentation remains up to date

#### Contained Item Comments
The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments. 

For exmaple if we wanted to add documentation to the crate `my_crate` we could add the following to `src/lib.rs`
```rs
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Notice there isn’t any code after the last line that begins with `//!`. Because we started the comments with `//!` instead of `///` we’re documenting the item that contains this comment rather than an item that follows this comment. In this case, that item is the `src/lib.rs` file, which is the crate root. These comments describe the entire crate.

When we run `cargo doc --open`, these comments will display on the front page of the documentation for my_crate above the list of public items in the crate.

#### Exporting a Convenient Public API

The structure of your public API is a major consideration when publishing a crate. People who use your crate are less familiar with the structure than you are and might have difficulty finding the pieces they want to use if your crate has a large module hierarchy.

-- continue here