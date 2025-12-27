## Chapter 15 – Smart Pointers: The Drop Trait

### Overview
The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.

You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.

We talk about Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped, it will deallocate the space on the heap that the box points to.

#### An Example of Drop
To see when Rust calls drop, let’s implement drop with println! statements for now.

The CustomSmartPointer struct's only custom functionality is that it will print Dropping CustomSmartPointer! when the instance goes out of scope, to show when Rust runs the drop method.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");
}
```

The output is the following:
```rs
CustomSmartPointers created
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```
Notice how they are dropped inversely to the order they are created. (Due to unwinding the stack)

#### Manual Drop
Rust allows you to manually drop a resource using `std::mem::drop`.

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main");
}
```

When we try to compile this code, we’ll get this error:
```rs
c.drop();
^^^^ explicit destructor calls not allowed
```

Rust doesn’t let us call drop explicitly, because Rust would still automatically call drop on the value at the end of main. This would cause a double free error because Rust would be trying to clean up the same value twice.

We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly. So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.