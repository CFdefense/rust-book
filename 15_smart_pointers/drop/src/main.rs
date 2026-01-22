/// A Custom Smart Pointer Type
/// 
/// Will serve to showcase the Drop Trait
/// 
struct CustomSmartPointer {
    data: String,
}

/// Implement the Drop Trait
/// 
/// FYI: The Drop trait is included in the prelude, so we don’t need to bring it into scope.
/// 
/// Will only print when dropped
/// 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // Lets create these two SmartPointers
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // Lets look at an example of dropping early using `drop()`
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main");
} // d and c will be dropped when out of scope
