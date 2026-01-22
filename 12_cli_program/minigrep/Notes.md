## Chapter 12 – `minigrep` CLI Program

### Reading Arguments
- The `minigrep` program reads command-line arguments to get the query and file name.

```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args[1] = query, args[2] = file name
}
```

### Searching and Errors
- Searching returns `Result` so the program can handle I/O errors gracefully.

```rs
fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
    // search contents...
    Ok(())
}
```


