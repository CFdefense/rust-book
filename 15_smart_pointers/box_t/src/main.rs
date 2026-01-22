/* A cons list entity
Gives error recursive type `List` has infinite size
enum List {
    Cons(i32, List),
    Nil,
}
*/

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // An example of creating a box for an i32 value
    let b = Box::new(5);
    println!("b = {b}");

    // Incorrect definition of a cons list
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // Proper definition of a cons list using a Box indirection
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}