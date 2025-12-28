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

Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert that it’s implemented correctly. This placeholder type is called a test double.

Mock objects are specific types of test doubles that record what happens during a test so that you can assert that the correct actions took place.

Here’s the scenario we’ll test: We’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make, for example.

Our library will only provide the functionality of tracking how close to the maximum a value is and what the messages should be at what times.

Applications that use our library will be expected to provide the mechanism for sending the messages: The application could show the message to the user directly, send an email, send a text message, or do something else. 

The library doesn’t need to know that detail. All it needs is something that implements a trait we’ll provide, called Messenger.

```rs
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}
```

We will then implement a mock object to instead of calling send will record the number of messages it was told to send


```rs
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

This test code defines a `MockMessenger` struct that has a `sent_messages` field with a Vec of String values to keep track of the messages it’s told to send. 

In the test, we’re testing what happens when the `LimitTracker` is told to set value to something that is more than **75** percent of the max value. However we see a compiler error:

```
cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
```

We can’t modify the `MockMessenger` to keep track of the messages, because the send method takes an immutable reference to self. 

We also can’t take the suggestion from the error text to use &mut self in both the impl method and the trait definition. We do not want to change the `Messenger` trait solely for the sake of testing.

This is a situation in which **interior mutability** can help! We’ll store the sent_messages within a `RefCell<T>`, and then the send method will be able to modify `sent_messages` to store the messages we’ve seen. 

```rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

The sent_messages field is now of type `RefCell<Vec<String>>` instead of `Vec<String>`. In the new function, we create a new `RefCell<Vec<String>>` instance around the empty vector.

For the implementation of the `send` method, the first parameter is still an immutable borrow of self, which matches the trait definition.

We call `borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a mutable reference to the value inside the `RefCell<Vec<String>>`, which is the vector. Then, we can call `push` on the mutable reference to the vector to keep track of the messages sent during the test.

The last change we have to make is in the assertion: To see how many items are in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference to the vector.

#### Tracking Borrows at Runtime

When creating immutable and mutable references, we use the **&** and **&mut** syntax, respectively.

With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`.

The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement **Deref**, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. 

Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.

In the following exmaple, we’re deliberately trying to create two mutable borrows active for the same scope to illustrate that `RefCell<T>` prevents us from doing this at runtime.

```rs
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // This makes two mutable references in the same scope, which isn’t allowed. Will panic!
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

#### Allowing Multiple Owners of Mutable Data
Another common way to use `RefCell<T>` is in combination with `Rc<T>`.

Recall that `Rc<T>` lets you have multiple owners of some data, but it only gives immutable access to that data.

If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!

Recall the cons list example from before where we used `Rc<T>` to allow multiple lists to share ownership of another list. `Rc<T>` holds only immutable values, we can’t change any of the values in the list once we’ve created them, so let’s add in `RefCell<T>` for its ability to change the values in the lists.

```rs
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}
```

We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a variable named value so that we can access it directly later. 

Then, we create a `List` in **a** with a `Cons` variant that holds value. We need to clone value so that both **a** and **value** have ownership of the inner **5** value rather than transferring ownership from value to a or having a borrow from value.

After we’ve created the lists in **a**, **b**, and **c**, we want to add **10** to the value in value, which we can do by getting a reference to the smart pointer with `borrow_mut()` and dereference it to get the mutable value.

This technique is pretty neat! By using `RefCell<T>`, we have an outwardly immutable List value. But we can use the methods on `RefCell<T>` that provide access to its interior mutability so that we can modify our data when we need to

Note that `RefCell<T>`does not work for multithreaded code! `Mutex<T>` is the thread-safe version of `RefCell<T>`.