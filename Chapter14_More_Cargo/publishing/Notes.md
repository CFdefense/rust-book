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

Users might also be annoyed at having to enter use `my_crate::some_module::another_module::UsefulType;` rather than use `my_crate::UsefulType;`.

The good news is that if the structure isn’t convenient for others to use from another library, you don’t have to rearrange your internal organization: Instead, you can re-export items to make a public structure that’s different from your private structure by using `pub use`.

Say we had an art library containing a module for **kinds** and another for **utils**

```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        unimplemented!();
    }
}
```

If we generated our docs we would notice that the `PrimaryColor` and `SecondaryColor` types aren’t listed on the front page, nor is the `mix` function. We have to click `kinds` and `utils` to see them.

Another crate would need to import them using the following:

```rs
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

They would need to figure out that `PrimaryColor` is in the `kinds` module and `mix` is in the `utils` module. 

To remove the internal organization from the public API, we can modify the art crate code using `pub use`:

```rs
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    // --snip--
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
```

The API documentation that cargo doc generates for this crate will now list and link re-exports on the front page.

The crate users can still see internal library structure but can now convienantly use our structures:

```rs
use art::PrimaryColor;
use art::mix;

fn main() {
    // --snip--
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

In cases where there are many nested modules, re-exporting the types at the top level with pub use can make a significant difference in the experience of people who use the crate. 


#### Setting Up a Crates.io Account

Before you can publish any crates, you need to create an account on `crates.io` and get an API key.

Then, run the cargo login command and paste your API key when prompted, like this:

```sh
$ cargo login
abcdefghijklmnopqrstuvwxyz012345
```

Define your crate name in `Cargo.toml`, must be unique:
```rs
[package]
name = "guessing_game"
```

You must also include other fields like **Metadata** and **License**
```rs
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

#### Publishing to Crates.io

Now that you’ve created an account, saved your API token, chosen a name for your crate, and specified the required metadata, you’re ready to publish!

Be careful, because a publish is permanent. The version can never be overwritten, and the code cannot be deleted except in certain circumstances.

To publish: run the `cargo publish` command

#### Publishing a New Version of an Existing Crate

When you’ve made changes to your crate and are ready to release a new version, you change the version value specified in your Cargo.toml file and republish. 

Use the **Semantic Versioning** rules to decide what an appropriate next version number is, based on the kinds of changes you’ve made. 

Then, run `cargo publish` to upload the new version.

#### Deprecating Versions from Crates.io