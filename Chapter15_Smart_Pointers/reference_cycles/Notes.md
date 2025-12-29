## Chapter 15 – Smart Pointers: Reference Cycles Can Leak Memory

### Overview
Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up.

Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.   

We can see that Rust allows memory leaks by using `Rc<T>` and `RefCell<T>`: It’s possible to create references where items refer to each other in a cycle.

This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

#### Creating a Reference Cycle

Let’s look at how a **reference cycle** might happen and how to prevent it, starting with the definition of the List enum and a tail method 

```rs
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

The second element in the Cons variant is now `RefCell<Rc<List>>`, meaning that instead of having the ability to modify the i32 value as we did before, we want to modify the List value a Cons variant is pointing to. 

We’re also adding a tail method to make it convenient for us to access the second item if we have a Cons variant.

Now lets use these definition in a main function:
```rs
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // The below line will show that we have a cycle;
    // it will overflow the stack.
    println!("a next item = {:?}", a.tail());
}
```

The reference count of the `Rc<List> `instances in both **a** and **b** is 2 after we change the list in **a** to point to **b**. At the end of main, Rust drops the variable **b**, which decreases the reference count of the **b** `Rc<List>` instance from **2** to **1**. 

The memory that `Rc<List>` has on the heap won’t be dropped at this point because its reference count is **1**, not **0**.

Compared to a real-world program, the consequences of creating a reference cycle in this example aren’t very dire: Right after we create the reference cycle, the program ends. 

However, if a more complex program allocated lots of memory in a cycle and held onto it for a long time, the program would use more memory than it needed and might overwhelm the system, causing it to run out of available memory. Always make sure to avoid cycling as Rust will not help us here.

#### Preventing Reference Cycles Using Weak<T>

