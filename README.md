# The Rust Book — Learning Rust

This repository documents my journey through [The Rust Programming Language](https://doc.rust-lang.org/book/), the official guide to learning Rust. Each file contains example programs, notes, and experiments that align with the chapters in the book.

## About the Rust Book

[The Rust Programming Language](https://doc.rust-lang.org/book/) is the official Rust guide, written by members of the Rust team. It provides a comprehensive, project-based introduction to Rust, covering everything from basic syntax to advanced language features like ownership, lifetimes, and concurrency.

## Repository Structure

Each file or directory corresponds to a chapter or section from the book. You'll find:

- Rust code that follows or expands on the book's examples
- Comments and notes explaining key concepts
- Additional examples or experiments to reinforce learning

## Getting Started

To run an example:

1. Install Rust using [rustup](https://rustup.rs/).
2. Navigate to the example’s directory.
3. Run it with Cargo:

```bash
cd chapter01_hello_cargo
cargo run
```

## Important Tip

If you're using Rust Analyzer in VS Code and want it to properly understand and lint examples within subdirectories (such as a chapter's Cargo project), you'll need to manually specify the current project in your workspace settings.

In .vscode/settings.json, set rust-analyzer.linkedProjects to point to the Cargo.toml file of the example you're working on:

```bash
{
  "rust-analyzer.linkedProjects": [
    "chapter08_common_collections/hashmaps/Cargo.toml"
  ]
}
```
