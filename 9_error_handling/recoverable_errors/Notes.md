## Chapter 9 – Recoverable Errors with `Result`

### `Result<T, E>`
- Functions can return `Result` to signal recoverable errors.

```rs
use std::fs::File;

fn open_config() -> Result<File, std::io::Error> {
    File::open("config.txt")
}
```

### Propagating Errors with `?`
- `?` returns early if the `Result` is `Err`.

```rs
use std::io::{self, Read};

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### Extra Notes (from chapter summary)
- Recoverable errors are handled by `Result<T, E>`.
- For some errors we may not want to crash the program.
- One way we can do this is through the `Result` enum, which is made up of `Err(E)` and `Ok(T)`.
- Instead of handling errors within some code you can return it to the calling code to deal with; this is called error propagation.
- The `?` operator for error propagation can only be used within a function whose return type is compatible with the value `?` is used on.



