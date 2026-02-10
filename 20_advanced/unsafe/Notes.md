## Chapter 20 - Advanced Features: Unsafe Rust

### Summary

**All** the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time. 

However, Rust has a *second language* hidden inside it that doesn’t enforce these memory safety guarantees: 

It’s called **unsafe Rust** and works just like regular Rust but gives us extra superpowers.

**Unsafe Rust** exists because, by nature, static analysis is **conservative**.

When the compiler tries to determine whether or not code upholds the guarantees, it’s **better** for it to reject *some valid programs* than to **accept** *some invalid programs*.

Although the code *might* be okay, if the Rust compiler doesn’t have enough information to be confident, it will **reject** the code.

In these cases, you can use `unsafe` code to tell the compiler, *“Trust me, I know what I’m doing.”*

If you use `unsafe` code **incorrectly**, problems can occur due to memory unsafety, such as null pointer dereferencing.

Another reason Rust has an `unsafe` alter ego is that the underlying computer hardware is **inherently unsafe**.

If Rust didn’t let you do `unsafe` operations, you *couldn’t* do certain tasks.

Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system.

### Performing Unsafe Superpowers

To switch to `unsafe` Rust, use the `unsafe` keyword and then start a new block that holds the `unsafe` code.

You can take **five actions** in unsaf`e Rust that you **can’t** in safe Rust, which we call *unsafe superpowers*.

These new superpower are:

1. Dereference a raw pointer.
2. Call an unsafe function or method.
3. Access or modify a mutable static variable.
4. Implement an unsafe trait.
5. Access fields of unions.

It’s important to understand that `unsafe` doesn’t turn off the borrow checker or disable any of Rust’s other safety checks:

For example: If you use a `reference` in `unsafe` code, it will **still** be checked.

The `unsafe` keyword *only gives you* access to these **five features** that are then not checked by the compiler for memory safety.

You’ll **still** get some degree of safety inside an unsafe block.

By using these `unsafe blocks` we can be sure where memory safety bugs live and as such we should try to keep these blocks short and concise.

Parts of the standard library are implemented as *safe abstractions* over *unsafe code* that has been audited.

### Dereferencing a Raw Pointer

In previous chapters we discussed how the compiler ensures that references are always **valid**.

`unsafe rust` introduces two new types called `raw pointers` that are similiar to `references`.

As with `references`, `raw pointers` can be immutable or mutable and are written as `*const T` and `*mut T` respectively.

Note: The *asterisk* is NOT a dereference operator, its actually part of the type name.

In the context of `raw pointers`, *immutable* means that the pointer can’t be directly assigned to after being dereferenced.

Different from `references` and `smart pointers`, `raw pointers`:

1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
2. Aren’t guaranteed to point to valid memory
3. Are allowed to be null
4. Don’t implement any automatic cleanup

By **opting out** of having Rust enforce these guarantees, you can give up guaranteed safety **in exchange** for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

Let's see an example of creating these `raw pointers`:

```rs
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
```

Notice that we **don’t** include the `unsafe` keyword in this code.

We **can** create `raw pointers` in *safe* code; we just **can’t dereference** `raw pointers` outside an `unsafe` block, as you’ll see in a bit.

We’ve created `raw pointers` by using the `raw borrow` operators: 

`&raw const num` creates a `*const i32` immutable `raw pointer`.
`&raw mut num` creates a `*mut i32` mutable `raw pointer`.

Because we created them directly from a local variable, we **know** these particular `raw pointers` are valid, but we **can’t** make that assumption about just any `raw pointer`.

To demonstrate this, next we’ll create a `raw pointer` whose validity we **can’t be so certain of**, using the keyword `as` to cast a value instead of using the `raw borrow` operator.

To showcase this we'll cast the value to an address specified in memory.

There *might* be data at that address or there *might* not, the compiler might optimize the code so that there is no memory access, or the program might terminate with a segmentation fault.

Usually, there is **no good reason** to write code like this, especially in cases where you can use a `raw borrow` operator instead, but it is possible:

```rs
    let address = 0x012345usize;
    let r = address as *const i32;
```

Recall that **we can create** raw pointers in safe code, but we **can’t dereference** raw pointers and read the data being pointed to.

Lets showcase this by using the dereference operator `*` on a raw pointer that requires an `unsafe block`:

```rs
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
```

Creating a pointer **does no harm**; it’s only when we try to **access the value** that it points at that we might end up dealing with an *invalid value*.

Note: we created `*const i32` and `*mut i32` raw pointers that both pointed to the s**ame memory location**, where `num` is stored.

If we instead tried to create an `immutable` and a `mutable` reference to `num`, the code would not have compiled.

With all of these dangers, **why** would you ever use raw pointers?

One major use case is when interfacing with **C code**.

Another case is when building up **safe abstractions** that the borrow checker doesn’t understand.

### Calling an Unsafe Function or Method

The second type of operation you can perform in an `unsafe block` is calling `unsafe functions`.

`Unsafe functions` and methods look exactly like regular functions and methods, but they have an extra `unsafe` before the rest of the definition.

The `unsafe` keyword in this context indicates the function has **requirements** we need to uphold when we call this function, because Rust can’t guarantee we’ve met these requirements. 

By calling an `unsafe function` within an `unsafe block`, we’re saying that we’ve read this **function’s documentation** and we **take responsibility** for upholding the function’s contracts.

Here is an `unsafe function` named `dangerous` that doesn’t do anything in its body:

```rs
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```

We must call the `dangerous` function within a separate `unsafe block`.

With the `unsafe block`, we’re asserting to Rust that we’ve read the function’s documentation, we understand how to use it properly, and we’ve verified that we’re fulfilling the contract of the function.

To perform `unsafe operations` in the body of an `unsafe function`, you **still** need to use an `unsafe block`, this helps us keep `unsafe blocks` *as small as possible*.

#### Creating a Safe Abstraction over Unsafe Code

Just because a function contains `unsafe code` **doesn’t** mean we need to mark the *entire function as unsafe*.

In fact, wrapping `unsafe code` in a `safe function` is a common abstraction.

As an example, let’s study the `split_at_mut` function from the **standard library**, which requires some `unsafe code`.

This `safe method` is defined on `mutable slices`: It takes one slice and makes it two by splitting the slice at the index given as an argument.

Heres an example of `split_at_mut` in use:

```rs
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
```

We **can’t** implement this function using only safe Rust.

For simplicity, we’ll implement `split_at_mut` as a function rather than a method and only for slices of `i32` values rather than for a generic type `T`.

```rs
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}
```

This function first gets the total length of the slice.

Then, it asserts that the index given as a parameter is within the slice by checking whether it’s less than or equal to the length.

Then, we return `two mutable slices` in a `tuple`: one from the start of the original slice to the mid index and another from mid to the end of the slice.

However, this gives us a compiler error:

```sh
$ cargo run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
error[E0499]: cannot borrow `*values` as mutable more than once at a time
 --> src/main.rs:6:31
  |
1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                         - let's call the lifetime of this reference `'1`
...
6 |     (&mut values[..mid], &mut values[mid..])
  |     --------------------------^^^^^^--------
  |     |     |                   |
  |     |     |                   second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*values` is borrowed for `'1`
  |
  = help: use `.split_at_mut(position)` to obtain two mutable non-overlapping sub-slices

For more information about this error, try `rustc --explain E0499`.
error: could not compile `unsafe-example` (bin "unsafe-example") due to 1 previous error
```

Rust’s borrow checker **can’t understand** that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice. 

Borrowing different parts of a slice is *fundamentally okay* because the two slices aren’t overlapping, but Rust isn’t smart enough to know this. 

When we know code is okay, but Rust doesn’t, it’s time to reach for `unsafe code`.

Lets see how to use an u`nsafe block`, a `raw pointer`, and some calls to u`nsafe functions` to make the implementation of `split_at_mut` work:

```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

A slice is a `pointer` to some data and the `length of the slice`.

We use the `len` method to get the *length of a slice* and the `as_mut_ptr` method to access the `raw pointer` of a slice.

In this case, because we have a *mutable slice* to `i32` values, `as_mut_ptr` returns a `raw pointer` with the type `*mut i32`, which we’ve stored in the variable ptr.

We keep the assertion that the mid index is within the slice.

Then, we get to the `unsafe code`: The `slice::from_raw_parts_mut` function takes a `raw pointer` and a `length`, and it creates a slice.

We use this function to create a slice that starts from `ptr` and is `mid` items long.

Then, we call the `add` method on `ptr` with `mid` as an argument to get a `raw pointer` that starts at `mid`, and we create a slice using that pointer and the remaining number of items after `mid` as the `length`.

The function `slice::from_raw_parts_mut` is `unsafe` because it takes a `raw pointer` and must *trust* that this pointer is **valid**. 

The `add` method on `raw pointers` is also `unsafe` because it must *trust* that the offset location is also a **valid pointer**. 

Therefore, we had to put an `unsafe` block around our calls to `slice::from_raw_parts_mut` and `add` so that we could call them.

Note that we don’t need to mark the resultant `split_at_mut` function as `unsafe`, and we can call this function from `safe Rust`.

We’ve created a `safe abstraction` to the `unsafe` code with an implementation of the function that uses `unsafe` code in a safe way, because it creates only valid pointers from the data this function has access to.

In contrast, the use of `slice::from_raw_parts_mut` in the following example would likely crash when the slice is used.

This code takes an arbitrary memory location and creates a slice *10,000* items long:

```rs
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

We don’t own the memory at this arbitrary location, and there is *no guarantee* that the slice this code creates contains valid `i32` values. 

Attempting to use values as though it’s a valid slice results in **undefined behavior**.

#### Using extern Functions to Call External Code

Sometimes your Rust code might need to interact with code written in *another language*.

For this, Rust has the keyword `extern` that facilitates the creation and use of a `Foreign Function Interface` (FFI). 

A `FFI` is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

The following example demonstrates how to set up an integration with the `abs` function from the *C standard library*. 

Functions declared within `extern` blocks are generally `unsafe` to call from Rust code, so `extern` blocks must also be marked `unsafe`.

The reason is that other languages *don’t enforce Rust’s rules* and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.

```rs
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Within the `unsafe extern "C" block`, we list the `names` and `signatures` of external functions from *another language* we want to call.

The `"C"` part defines which `application binary interface` (ABI) the external function uses.

The `ABI` defines how to call the function at the **assembly level**. 

The `"C" ABI` is the most common and follows the `C programming language’s ABI`.

Every item declared within an `unsafe extern block` is implicitly `unsafe`. 

However, *some* `FFI functions` are **safe** to call.

For example, the `abs` function from C’s standard library does not have any memory safety considerations, and we know it can be called with **any** `i32`.

In cases like this, we can use the `safe` keyword to say that this specific function is **safe** to call even though it is in an `unsafe extern block`.

Once we make that change, calling it no longer requires an `unsafe block`.

```rs
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}
```

Note: Marking a function as `safe` **does not inherently make it safe**!

Instead, it is like a *promise* you are making to Rust that it is safe. It is still *your responsibility* to make sure that promise is kept!

#### Calling Rust Functions from Other Languages

We can also use `extern` to create an *interface* that allows other languages to call Rust functions.

Instead of creating a whole `extern block`, we add the `extern` keyword and specify the `ABI` to use just before the `fn` keyword for the relevant function.

We also need to add an `#[unsafe(no_mangle)]` annotation to tell the Rust compiler not to `mangle` the name of this function.

`Mangling` is when a compiler changes the name we’ve given a function to a **different name** that contains more information for other parts of the compilation process to consume but is *less human readable*.

This is `unsafe` because there might be **name collisions** across libraries without the built-in mangling, so it is our responsibility to make sure the **name** we choose is safe to export without mangling.

In the following example, we make the `call_from_c` function accessible from `C code`, after it’s compiled to a shared library and linked from C:

```rs
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

This usage of `extern` requires `unsafe` only in the attribute, not on the `extern block`.

### Accessing or Modifying a Mutable Static Variable

In this book, we’ve not yet talked about `global variables`, which Rust *does support* but which can be **problematic** with Rust’s ownership rules.

If two threads are accessing the same `mutable global variable`, it can cause a data race.

In Rust, `global variables` are called `static variables`.

Here is a simple example declaration and use of a `static variable` with a string slice as a value.

```rs
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("value is: {HELLO_WORLD}");
}
```

The names of `static variables` are in SCREAMING_SNAKE_CASE by convention

`Static` variables can only store references with the `'static lifetime`, which means the Rust compiler *can figure out the lifetime* and we aren’t required to annotate it explicitly.

Accessing an `immutable static variable` is safe.

A subtle difference between `constants` and `immutable static variables` is that values in a `static variable` have a *fixed address* in memory. 

Using the value will always access the **same data**.

`Constants`, on the other hand, are *allowed* to duplicate their data whenever they’re used.

Another difference is that `static variables` can be `mutable`.

*Accessing and modifying* `mutable static variables` is `unsafe`.

Lets demonstrate how to declare, access, and modify a mutable static variable named `COUNTER`.

```rs
static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}
```

As with regular variables, we specify mutability using the `mut` keyword.

Any code that *reads or writes* from `COUNTER` must be within an `unsafe block`.

Having *multiple threads* access `COUNTER` would likely result in data races, so it is **undefined behavior**.

Therefore, we need to mark the entire function as `unsafe` and document the `safety limitation` so that anyone calling the function knows what they are and are not allowed to do safely.

Whenever we write an `unsafe function`, it is **idiomatic** to write a comment starting with `SAFETY` and explaining what the caller needs to do to call the function safely.

Additionally, the compiler will **deny** by default any attempt to create references to a `mutable static variable` through a compiler lint.

You must either explicitly **opt out** of that lint’s protections by adding an `#[allow(static_mut_refs)]` annotation 

or 

access the `mutable static variable` via a `raw pointer` created with one of the `raw borrow` operators. 

That includes cases where the reference is created **invisibly**, as when it is used in the `println!` in this code listing.

Requiring references to `static mutable variables` to be created via `raw pointers` helps make the safety requirements for using them more **obvious**.

With `mutable data` that is **globally accessible**, it’s difficult to ensure that there are no data races, which is why Rust considers `mutable static variables` to be `unsafe`.

Where possible, it’s preferable to use Rust's **concurrency techniques** and `thread-safe smart pointers`.

### Implementing an Unsafe Trait

We can use `unsafe` to implement an `unsafe trait`.

A `trait` is `unsafe` when at least **one of its methods** has some `invariant` that the compiler **can’t verify**.

We declare that a `trait` is `unsafe` by adding the `unsafe` keyword before `trait` and marking the implementation of the `trait` as `unsafe` too:

```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

By using `unsafe impl`, we’re *promising* that we’ll uphold the invariants that the compiler can’t verify.

As an example, recall the `Send` and `Sync` marker traits.

The compiler implements these traits **automatically** if our types are composed entirely of other types that implement `Send` and `Sync`.

If we implement a type that contains a type that does not implement `Send` or `Sync`, such as `raw pointers`, and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. 

Rust **can’t verify** that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads.

Therefore, we need to do those checks **manually** and indicate as such with `unsafe`.

### Accessing Fields of a Union

The final action that works only with `unsafe` is accessing fields of a `union`.

A `union` is similar to a `struct`, but *only one declared field is used* in a particular instance at one time.

`Unions` are primarily used to interface with `unions` in *C code*.

Accessing `union` fields is `unsafe` because Rust *can’t guarantee* the type of the data currently being stored in the `union` instance.

### Using Miri to Check Unsafe Code

When writing unsafe code, you might want to check that what you have written *actually* is safe and correct.

One of the best ways to do that is to use `Miri`, an **official** Rust tool for detecting undefined behavior.

Whereas the borrow checker is a static tool that works at **compile time**, `Miri` is a dynamic tool that works at **runtime**.

It checks your code by running your program, or its test suite, and detecting when you violate the rules it understands about how Rust should work.

Using `Miri` requires a `nightly build` of Rust.

You can install both a `nightly version` of Rust and the `Miri` tool by typing `rustup +nightly component add miri`.

This does not change what version of Rust your project uses; it only adds the `tool` to your system so you can use it when you want to.

You can run `Miri` on a project by typing `cargo +nightly miri run` or `cargo +nightly miri test`.

Here is `Miri` ran on one of our previous undefined behavior examples:

```sh
$ cargo +nightly miri run
   Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `file:///home/.rustup/toolchains/nightly/bin/cargo-miri runner target/miri/debug/unsafe-example`
warning: integer-to-pointer cast
 --> src/main.rs:5:13
  |
5 |     let r = address as *mut i32;
  |             ^^^^^^^^^^^^^^^^^^^ integer-to-pointer cast
  |
  = help: this program is using integer-to-pointer casts or (equivalently) `ptr::with_exposed_provenance`, which means that Miri might miss pointer bugs in this program
  = help: see https://doc.rust-lang.org/nightly/std/ptr/fn.with_exposed_provenance.html for more details on that operation
  = help: to ensure that Miri does not miss bugs in your program, use Strict Provenance APIs (https://doc.rust-lang.org/nightly/std/ptr/index.html#strict-provenance, https://crates.io/crates/sptr) instead
  = help: you can then set `MIRIFLAGS=-Zmiri-strict-provenance` to ensure you are not relying on `with_exposed_provenance` semantics
  = help: alternatively, `MIRIFLAGS=-Zmiri-permissive-provenance` disables this warning
  = note: BACKTRACE:
  = note: inside `main` at src/main.rs:5:13: 5:32

error: Undefined Behavior: pointer not dereferenceable: pointer must be dereferenceable for 40000 bytes, but got 0x1234[noalloc] which is a dangling pointer (it has no provenance)
 --> src/main.rs:7:35
  |
7 |     let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
  |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
  |
  = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
  = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
  = note: BACKTRACE:
  = note: inside `main` at src/main.rs:7:35: 7:70

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to 1 previous error; 1 warning emitted
```

`Miri` *correctly* warns us that we’re casting an integer to a pointer, which might be a problem, but `Miri` can’t determine whether a problem exists because it doesn’t know how the pointer originated.

`Miri` returns an error as the program has undefined behavior because we have a dangling pointer.

`Miri` *doesn’t catch everything* you might get wrong when writing `unsafe` code. 

`Miri` is a *dynamic analysis tool*, so it only catches problems with code that actually gets run.

`Miri` also does not cover *every possible way* your code can be unsound.

Put another way: If `Miri` does catch a problem, you **know** there’s a bug, but just because `Miri` doesn’t catch a bug **doesn’t** mean there isn’t a problem.

### Using Unsafe Code Correctly

Using `unsafe` to use one of the five superpowers just discussed *isn’t wrong* or even *frowned upon*, but it is **trickier** to get `unsafe` code correct because the compiler can’t help uphold memory safety.

When you have a reason to use `unsafe` code, you can do so, and having the explicit `unsafe` annotation makes it easier to track down the source of problems when they occur.

Whenever you write `unsafe` code, you can use `Miri` to help you be more confident that the code you have written upholds Rust’s rules.