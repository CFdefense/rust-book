## Chapter 14 – More About Cargo and Crates.io: Installing Binaries with cargo install

The cargo install command allows you to install and use binary crates locally. 

Note: This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on `crates.io`. 

Note that you can only install packages that have binary targets.

A binary target is the runnable program that is created if the crate **has** a `src/main.rs` file or another file specified as a binary, as opposed to a **library** target that isn’t runnable on its own but is suitable for including within other programs. 

All binaries installed with cargo install are stored in the installation root’s bin folder. If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be `$HOME/.cargo/bin`.

Example:

```sh
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--snip--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```