## Chapter 15 – Smart Pointers: Rc<T>

### Overview
In the majority of cases, ownership is clear: You know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners. 

For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for reference counting.

The `Rc<T>` type keeps track of the number of immutable references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

#### Use Case
We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. 

If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

Note that Rc<T> is only for use in single-threaded scenarios.

#### Sharing Data

Let’s return to our cons list example from Box<T>. This time, we’ll create two lists that both share ownership of a third list.

We’ll create list **a** that contains **5** and then **10**.

Then, we’ll make two more lists: **b** that starts with **3** and **c** that starts with **4**.

Both the **b** and **c** lists will then continue on to the first **a** list containing **5** and **10**.

In other words, both lists **b** and **c** will share the first list **a** containing **5** and **10**.

##### First Attempt

```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

This code will not compile due to moved value:
```
use of moved value: `a`
```

The Cons variants own the data they hold, so when we create the **b** list, **a** is moved into **b** and **b** owns **a**. Then, when we try to use **a** again when creating **c**, we’re not allowed to because **a** has been moved.

We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters. 

By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list.

##### Second Attempt Using Rc<T>

Let's change our definition of List to use `Rc<T>` in place of `Box<T>`, each Cons variant will now hold a value and an `Rc<T>` pointing to a List.

When we create **b**, instead of taking ownership of **a**, we’ll clone the `Rc<List>` that **a** is holding, thereby increasing the number of references from one to two and letting **a** and **b** share ownership of the data in that `Rc<List>`.

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

Every time we call `Rc::clone`, the reference count to the data within the `Rc<List>` will increase, and the data won’t be cleaned up unless there are zero references to it.

We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust’s convention is to use `Rc::clone` in this case. `Rc::clone` will only implement the Rc count it will not make a deep copy, so it is faster.

#### Cloning to Increase the Reference Count

Let's Change our approach so we can see the References changing as we increment and decrement the references to the `Rc<List>`: **a**.

We’ll change main so that it has an inner scope around list **c**; then, we can see how the reference count changes when **c** goes out of scope.

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// --snip--

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

At each point in the program where the reference count changes, we print the reference count, which we get by calling the `Rc::strong_count` function.

This function is named `strong_count` rather than count because the `Rc<T>` type also has a `weak_count`. We'll look into `weak_count` later.

The implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.

What we can’t see in this example is that when **b** and then **a** go out of scope at the end of main, the count is **0**, and the `Rc<List>` is cleaned up completely.

Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.