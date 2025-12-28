## Chapter 15 – Smart Pointers: RefCell<T>

### Overview
Interior mutability is a design pattern in Rust that allows you to **mutate** data even when there are **immutable** references to that data; normally, this action is disallowed by the borrowing rules. 

To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

We can use types that use the `interior mutability` pattern only when we can **ensure** that the borrowing rules will be followed at runtime, even though the compiler can’t **guarantee** that. 

#### Enforcing Borrowing Rules at Runtime

Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>?

**The Difference** is with references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at **runtime**. 

With references, if you break these rules, you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will panic and exit.

The advantage of checking the borrowing rules at **runtime** instead is that certain memory-safe scenarios are then **allowed**, where they would’ve been disallowed by the compile-time checks. 

Rust's static code analysis in its conversative nature is imperfect, some memory safe programs will be rejected. 

The `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

#### Which Pointer Should I Use?
Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

- `Rc<T>` enables multiple owners of the same data; `Box<T>`and `RefCell<T>` have single owners.
- `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
- `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

#### The Interior Mutability Pattern
A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably. For example, this code won’t compile:

```rs
fn main() {
    let x = 5;
    let y = &mut x;
}
```

This will result in the following error:
```
cannot borrow `x` as mutable, as it is not declared as mutable
```

However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code, this is called Interior Mutability. One way Interior Mutability can be accomplished is by using `RefCell<T>`.

#### A Practical Example
