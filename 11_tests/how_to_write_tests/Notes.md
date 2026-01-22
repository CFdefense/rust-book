## Chapter 11 – How to Write Tests

### Basic Test Function
- Mark test functions with `#[test]` and run them with `cargo test`.

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

### Assertions
- Use `assert!`, `assert_eq!`, and `assert_ne!` to check conditions in tests.

### Extra Notes (from chapter summary – writing tests)
- At its simplest, a test in Rust is a function that’s annotated with the `test` attribute.
- Attributes are metadata about pieces of Rust code; one example is the `derive` attribute used with structs in Chapter 5.
- To change a function into a test function, add `#[test]` on the line before `fn` and run with `cargo test`.
- You can use `assert_eq!` or `assert_ne!` for testing equality or inequality instead of writing `==` inside `assert!`.
- You can write custom messages for assertions.
- You can denote that a test is expected to fail with `#[should_panic]`.
- You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`.
- To assert that an operation returns an `Err` variant, don’t use the `?` operator on the `Result<T, E>` value; instead, use `assert!(value.is_err())`.

### Extra Notes (from chapter summary – controlling how tests run)
- `cargo test` compiles all tests and runs them in parallel by default.
- We must be careful with parallelism, because tests that interact with the same environment can interfere with each other.
- You can avoid running in parallel with the flag `-- --test-threads=1`.
- To see print statements of passing tests (hidden by default), use `cargo test -- --show-output`.
- You can pass the name of any individual test to run only that test, e.g. `cargo test some_test_name`.
- You can filter and run tests with similar names, e.g. `cargo test test_pems`.
- You can ignore expensive tests by adding the `#[ignore]` attribute and then run only ignored tests with `cargo test -- --ignored`.


