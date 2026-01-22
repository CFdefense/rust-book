enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Well first create a Cons list of 5,10 using Rc<T>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // We created a with Rc<T> so we can have b and c below point to a without taking ownership
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}