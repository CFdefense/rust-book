/*
-Ownership rules
    -Each value in Rust has an owner.
    -There can only be one owner at a time.
    -When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    /* Scope */
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    /* Complex Mutable Data Type Allocated To Heap */
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`

    /* Here when we go to move the value - it is simply copied in its entirety

       This is whats called a deep copy. However we do not need .clone() due to the low cost of this with ints.
    */
    let x = 5;
    let y = x;

    /* However, if we want to move a heap allocated type - We copy the ptr, len and capacity
       We do this becuase it is less expensive than allocating more of the heap for an already
       existing heap object.
    */

    /* Important to understand that after being copied s1 is no longer valid.
       It is no longer valid because when 'free' is called at the end, it would call on both s1 and s2.
       This would cause a double free error. Therefore rust says s1 is no longer valid.
       If you would want to use s1 rust will tell you to use .clone() when you try to move.

       This is whats called a shallow copy, calling .clone() is a deep copy
    */
    let s1 = String::from("hello");
    let s2 = s1;

    /* In this case the old allocated heap address for 'hello' is immediately dropped
       We then go ahead and create and allocate the new space for 'ahoy'
    */
    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

/* Can return many values using a tuple */
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
