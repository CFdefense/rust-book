## Chapter 11 – Organizing Tests

### Unit Tests
- Unit tests live next to the code in `src` and are placed in a `tests` module with `#[cfg(test)]`.

```rs
#[cfg(test)]
mod tests {
    // unit tests here
}
```

### Integration Tests
- Integration tests go in the top-level `tests` directory and use the crate like any external user.

```rs
// tests/integration_tests.rs
use your_crate_name::*;
```

### Extra Notes (from chapter summary – test organization)
- The main Rust consensus is to split tests into unit tests and integration tests.
- Unit tests are small and focused, testing one module in isolation at a time, and can test private interfaces.
- Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface.
- The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
- You’ll put unit tests in the `src` directory in each file with the code that they’re testing.
- The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `#[cfg(test)]`.
- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`.
- There’s debate about whether or not private functions should be tested directly, but Rust’s privacy rules do allow you to test private functions.
- You create a `tests` directory at the top level of your project; Cargo knows to look there for integration test files.
- You can run a specific integration test using `cargo test --test integration_test`.
- Because each module in `tests/` is treated as its own crate, you can use `tests/common/mod.rs` to store shared setup utilities for tests.

