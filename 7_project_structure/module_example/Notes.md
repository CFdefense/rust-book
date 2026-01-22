## Chapter 7 – Module Example

### Module Tree and Files
- The crate root (`src/main.rs`) declares modules with `mod`.
- Rust maps module paths to files and folders.

```rs
mod garden; // compiler looks for garden.rs or garden/mod.rs
```

### Submodules
- Inside `garden.rs` you can declare submodules like:

```rs
pub mod vege; // looks for garden/vege.rs or garden/vege/mod.rs
```


