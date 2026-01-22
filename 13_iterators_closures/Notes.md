## Chapter 13 – Closures and Iterators

These notes have been distributed to:

- `closures/Notes.md`
- `iterators/Notes.md`

Use this file as a quick index to the subchapter notes.

### Extra Notes (from chapter summary)
To determine whether to use loops or iterators, you need to know which implementation is faster: the version of the search function with an explicit for loop or the version with iterators.

Essentially: The two implementations have similar performance!

Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. 

Iterators are one of Rust’s zero-cost abstractions, by which we mean that using the abstraction imposes no additional runtime overhead. 