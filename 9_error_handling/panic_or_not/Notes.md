## Chapter 9 – To Panic or Not to Panic

### When to Return `Result`
- Use `Result` when the caller can reasonably handle the error.
- Examples: file not found, invalid user input.

### When to Panic
- Use `panic!` when continuing would be incorrect or unsafe.
- Examples: internal invariants broken, bugs that should never happen.

### Extra Notes (from chapter summary)
- It is good practice to really consider when we should be calling `panic!` vs returning a `Result`.
- After returning a `Result`, we can still decide to panic if we really need to, but we give the program more options.
- Generally it is good practice to panic when your program enters some unknown state, whether variable types or logic becomes undetermined.
- When failure is acceptable and expected it is a better practice to return a `Result` to handle it.



