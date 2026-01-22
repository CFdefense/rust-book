## Chapter 9 – Unrecoverable Errors with `panic!`

### `panic!` Macro
- Use `panic!` when the program cannot continue in a valid state.

```rs
fn main() {
    panic!("crash and burn");
}
```

### Backtraces
- `RUST_BACKTRACE=1 cargo run` shows where the panic happened.

### Extra Notes (from chapter summary)
- Rust has two types of errors, unrecoverable errors and recoverable errors.
- Unrecoverable errors are handled by the `panic!` macro.
- Recoverable errors are handled by `Result<T, E>`.
- `"RUST_BACKTRACE=1 cargo run"` is an example of how we can get more verbose debugging with a stack trace.



