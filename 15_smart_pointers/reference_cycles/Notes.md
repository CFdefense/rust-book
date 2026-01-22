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

So far, we’ve demonstrated that calling `Rc::clone` increases the `strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is **0**.

You can also create a weak reference to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>.`

**Strong references** are how you can share ownership of an Rc<T> instance.

**Weak references** don’t express an ownership relationship, and their count doesn’t affect when an `Rc<T>` instance is cleaned up. They won’t cause a reference cycle, because any cycle involving some weak references will be broken once the strong reference count of values involved is **0**.

When you call `Rc::downgrade`, you get a smart pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the `Rc<T>` instance by **1**, calling `Rc::downgrade` increases the `weak_count` by **1**.

The **key** difference is the `weak_count` doesn’t need to be **0** for the `Rc<T>` instance to be cleaned up.

Because the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to you must make sure the value still exists. Do this by calling the `upgrade` method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`.

#### Tree Example

To start, we’ll build a tree with nodes that know about their child nodes. We’ll create a struct named `Node` that holds its own `i32` value as well as references to its child `Node` values:

```rs
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

We want a Node to own its children, and we want to share that ownership with variables so that we can access each Node in the tree directly.

We also want to modify which nodes are children of another node, so we have a `RefCell<T>` in children around the `Vec<Rc<Node>>`.

#### Adding a Reference from a Child to Its Parent

To make the child node aware of its parent, we need to add a parent field to our Node struct definition.

The trouble is in deciding what the **type** of parent should be. We know it can’t contain an `Rc<T>`, because that would create a reference cycle with `leaf.parent` pointing to branch and `branch.children` pointing to leaf, which would cause their `strong_count` values to never be **0**.

Thinking about the relationships another way, a parent node should own its children: If a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: If we drop a child node, the parent should still exist. This is a case for weak references!

So, instead of `Rc<T>`, we’ll make the type of parent use `Weak<T>`, specifically a` RefCell<Weak<Node>>`. Now our Node struct definition looks like this:

```rs
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

A node will be able to refer to its parent node but doesn’t own its parent. 

#### Visualizing Changes to strong_count and weak_count

Let’s look at how the `strong_count` and `weak_count` values of the `Rc<Node>` instances change by creating a new inner scope and moving the creation of branch into that scope.

```rs
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

After leaf is created, its `Rc<Node>` has a strong count of **1** and a weak count of **0**. 

In the inner scope, we create branch and associate it with leaf, at which point when we print the counts, the `Rc<Node>` in branch will have a strong count of **1** and a weak count of **1**.

When we print the counts in leaf, we’ll see it will have a strong count of **2** because branch now has a clone of the `Rc<Node>` of leaf stored in `branch.children` but will still have a weak count of **0**.

When the inner scope ends, branch goes out of scope and the strong count of the `Rc<Node>` decreases to **0**, so its Node is dropped. 

The weak count of **1** from `leaf.parent` has no bearing on whether or not Node is dropped, so we don’t get any memory leaks!

If we try to access the parent of leaf after the end of the scope, we’ll get None again.

All of the logic that manages the counts and value dropping is built into `Rc<T>` and `Weak<T>` and their implementations of the `Drop` trait. 