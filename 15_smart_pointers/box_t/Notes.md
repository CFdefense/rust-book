## Chapter 15 – Smart Pointers: Box<T>

### Overview
The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack. 

#### When to use heap allocation
Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. You’ll use them most often in these situations:
- When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size.
- When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so.
- When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type.

#### Storing Data on the Heap
```rs
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```
Above is an example of storing an i32 value on the heap.

The `box` is the pointer stored in the stack.
The `5` is an i32 value, it will be stored in the heap and the internal `box` pointer will point to it.

This approach does not entirely make sense to do as storing there is virtually no benefit to storing an i32 on the heap instead of the stack.

**Let's look at some better examples**

#### Enabling Recursive Types with Boxes

A value of a `recursive type` can have another value of the same type as part of itself. 

**Problem**
Recursive types pose an issue in Rust because Rust needs to know at compile time how much space a type takes up.

**Solution**
Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

We'll explore how to do this using a recursive example with a `Cons list`

#### Cons list
A cons list is a data structure that comes from the Lisp programming language and its dialects, is made up of nested pairs, and is the Lisp version of a linked list.

For example, here’s a pseudocode representation of a cons list containing the list 1, 2, 3 with each pair in parentheses:
```rs
(1, (2, (3, Nil)))
```

Each item in a cons list contains two elements: the value of the current item and of the next item. The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function (`construct` in lisp).

Lets use the following type to represent a single cons list pair:
```rs
enum List {
    Cons(i32, List),
    Nil,
}
```

If we then try to create and compile the following cons list:
```rs
fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

We will see an error for `recursive type 'List' has infinite size` due to Rust not being able to determine how much space it needs to store a `List` value.

#### How Does Rust Determine Type Size?
Recall type `Messsage`

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

To determine how much space to allocate for a Message value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth. Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.

Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum in Listing 15-2 needs. The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List. Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List. 

This leads to a recursive type inquiry.

#### Fix using indirection

Indirection means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: A pointer’s size doesn’t change based on the amount of data it’s pointing to. This will fix our unknown size issue.

Now lets apply this fix to our previous example: 
```rs
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

#### Summmary
Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types. They also don’t have the performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.

The Box<T> type is a smart pointer because it implements the `Deref` trait, which allows Box<T> values to be treated like references. 

When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.