/// Simple Box<T> Clone struct
/// 
/// Holds Generic type T
/// 
struct MyBox<T>(T);

/// MyBox<T> Box<T> clone Constructor
/// 
/// Holds Generic type T
/// 
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

/// Implementation of Trait Deref for MyBox
/// 
/// Requires an implementation of 'deref'
/// 
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(message: &str) {
    print!("{}", message)
}

fn main() {
    // x contains the i32 value 5
    let x = 5;

    // y is a Box containing the copied x
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // This line will only work if Deref is implemented for MyBox

    // Lets showcase Deref coercion
    let message = MyBox::new(String::from("Hello"));

    // Despite hello taking a reference to string slice it will accept our Box of String due to Deref coersion
    hello(&message);

    // If Rust did not implement Deref coersion the following code would be needed:
    // hello(&(*m)[..]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reference() {
        // x contains the i32 value 5
        let x = 5;

        // y is a reference to x
        let y = &x;

        // We can simply assert 5 is x
        assert_eq!(5, x);

        // However for y we must deference it first to assert its value hiding behind the reference
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box_reference() {
        // x contains the i32 value 5
        let x = 5;

        // y is a Box containing the copied x
        let y = Box::new(x);

        // We can simply assert 5 is x
        assert_eq!(5, x);

        // However for y we must still deference it first to assert its value hiding behind the box
        assert_eq!(5, *y);
    }
}