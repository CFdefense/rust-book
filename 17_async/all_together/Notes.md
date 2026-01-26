## ## Chapter 17 - Asynchronous Programming: Putting It All Together: Futures, Tasks, and Threads

### Overview

As we saw in Chapter 16, one approach to concurrency is through **Threads**.

However, weve seen a new approach in this chapter using `async` with `futures` and `streams`.

So when do we use `threads` and when do we use `async`, the answer is *it depends*.

Sometimes the best approach is using them both together!

`Threads` have some downsides such as memory usage and requiring hardware and an `OS` that supports them.

`Async` uses tasks to complete its work, a `task` is similiar to a thread but instead of being managed by the `OS` its managed by the `runtime`.

There’s a reason the APIs for spawning threads and spawning tasks are so similar.