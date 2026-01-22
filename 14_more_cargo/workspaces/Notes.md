## Chapter 14 – More About Cargo and Crates.io: Cargo Workspaces

In Chapter 12, we built a package that included a binary crate and a library crate. As your project develops, you might find that the library crate continues to get bigger and you want to split your package further into multiple library crates. Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.

#### Creating a Workspace

A **workspace** is a set of packages that share the same `Cargo.lock` and `output` directory.

Let's create a workspace, our main binary will depend on **two** libraries: One library will provide an `add_one` function and the other library an `add_two` function. 

These three crates will be part of the same workspace. We’ll start by creating a new directory for the workspace:

Create a new directory for the workspace:

```sh
$ mkdir add
$ cd add
```

Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace. 

This file won’t have a `[package]` section. Instead, it will start with a `[workspace]` section that will allow us to add members to the workspace. 

We also make a point to use the latest and greatest version of Cargo’s `resolver` algorithm in our workspace by setting the resolver value to **"3"**:

```rs
[workspace]
resolver = "3"
```

Next, we’ll create the adder binary crate by running cargo new within the add directory:
```sh
$ cargo new adder
     Created binary (application) `adder` package
      Adding `adder` as member of workspace at `file:///projects/add`
```

And add it to our workspace:
```sh
[workspace]
resolver = "3"
members = ["adder"]
```

At this point, we can build the workspace by running cargo build. The files in your add directory should look like this:
```sh
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one target directory at the top level that the compiled artifacts will be placed into; the adder package doesn’t have its own target directory.

Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in `add/target` rather than `add/adder/target`.

By sharing one target directory, the crates can avoid unnecessary rebuilding. In addition they should rely on eachother.


#### Creating the Second Package in the Workspace

Next, let’s create another member package in the workspace and call it `add_one`. Generate a new library crate named `add_one`:

```sh
$ cargo new add_one --lib
     Created library `add_one` package
      Adding `add_one` as member of workspace at `file:///projects/add`
```

And we'll also add it to the workspace members:

```rs
[workspace]
resolver = "3"
members = ["adder", "add_one"]
```

Now `add` should look like this:
```sh
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

In the `add_one/src/lib.rs` file, let’s add an `add_one` function:

```rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Now we can have the `adder` package with our binary depend on the `add_one` package that has our library. First, we’ll need to add a path dependency on `add_one` to `adder/Cargo.toml`.

```rs
[dependencies]
add_one = { path = "../add_one" }
```

Next, let’s use the `add_one` function (from the add_one crate) in the adder crate. Open the `adder/src/main.rs` file and change the main function to call the `add_one` function

```rs
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

We can now build the workspace:

```sh
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

We can then choose which package to run using the **-p** flag

```sh
$ cargo run -p adder
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

This runs the code in adder/src/main.rs, which depends on the add_one crate.

#### Depending on an External Package

Notice that the workspace has only one Cargo.lock file at the top level, rather than having a Cargo.lock in each crate’s directory. This ensures that all crates are using the **same** version of all dependencies.

If we add the `rand` package to the `adder/Cargo.toml` and `add_one/Cargo.toml` files, Cargo will resolve both of those to one version of `rand` and record that in the one `Cargo.lock`.

Let’s add the `rand` crate to the `[dependencies]` section in the `add_one/Cargo.toml` file so that we can use the `rand` crate in the `add_one` crate:

```rs
[dependencies]
rand = "0.8.5"
```

However, even though `rand` is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add `rand` to their `Cargo.toml` files as well. 

For example, if we add u`se rand;` to the `adder/src/main.rs` file for the adder package, we’ll get an error:

```sh
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

To fix this, edit the Cargo.toml file for the adder package and indicate that rand is a dependency for it as well. 

If crates in the workspace specify incompatible versions of the same dependency, Cargo will resolve each of them but will still try to resolve as few versions as possible.

#### Adding a Test to a Workspace

For another enhancement, let’s add a test of the `add_one::add_one` function within the `add_one` crate:

```rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Running `cargo test` will find and run all tests for all crates in the workspace.

We can also specify a package to run tests on using the **-p** flag

```sh
$ cargo test -p add_one
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-93c49ee75dc46543)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

As your project grows, consider using a workspace: It enables you to work with smaller, easier-to-understand components than one big blob of code. 

Furthermore, keeping the crates in a workspace can make coordination between crates easier if they are often changed at the same time.
