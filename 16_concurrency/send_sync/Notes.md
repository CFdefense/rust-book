## ## Chapter 16 - Fearless Concurrency: Extensible Concurrency with Send and Sync

### Overview

Interestingly, almost every concurrency feature we’ve talked about so far in this chapter has been part of the standard library, not the language. 

However, among the key concurrency concepts that are embedded in the language rather than the standard library are the `std::marker` traits `Send` and `Sync`.

#### Transferring Ownership Between Threads

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. 

Almost every Rust type implements `Send`, but there are some exceptions, including `Rc<T>`:

`Rc<T>` cannot implement `Send` because if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time. 

Any type composed entirely of `Send` types is automatically marked as `Send` as well.

Almost all primitive types are `Send`, aside from raw pointers.

#### Accessing from Multiple Threads

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads.

In other words, any type **T** implements `Sync` if **&T** (an immutable reference to T) implements `Send`, meaning the reference can be sent safely to another thread. 

Similar to `Send`, primitive types all implement `Sync`, and types composed entirely of types that implement `Sync` also implement `Sync`.

The smart pointer `Rc<T>` also doesn’t implement `Sync` for the same reasons that it doesn’t implement `Send`.

#### Implementing Send and Sync Manually Is Unsafe

Because types composed entirely of other types that implement the `Send` and `Sync` traits also automatically implement `Send` and `Sync`, we **don’t** have to implement those traits manually.

As **marker traits**, they don’t even have any methods to implement. They’re just useful for enforcing invariants related to concurrency.

Manually implementing these traits involves implementing `unsafe` Rust code.