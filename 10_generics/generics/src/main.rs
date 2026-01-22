// as see below we have different functions which do basically the same thing for i32 and char

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we can implement this function to use generic type T instead of int or char
// it will perform the same logic but be applicable to any type (or will it)
// it will not compile due to trait std::cmp::PartialOrd restrictions
// we cannot compare all types with '>' only those which implement std::cmp::PartialOrd

fn largest<T>(list: &[T]) -> &T {
    let mut _largest = &list[0];

    // for item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    _largest
}

// we can also implement types to use generics 
struct Point<T> {
    x: T,
    y: T,
}

// lets implement a generic method on the generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// we dont always have to match types aswell
// in this definition we declare X1, Y1
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

// but in the method mixup we use X2, Y2
impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

// we can also restict methods to certain types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// we can also use multiple generics
struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    // see redundant functions below
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    // now lets use the generic function
    let result = largest(&char_list);
    println!("The largest Type T is {result}");

    // create a object of the struct with generics
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // all types of type T must be the same
    // let wont_work = Point { x: 5, y: 4.0 };

    // use multiple generics
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // use generic method
    println!("p.x = {}", integer.x());

    // use differing generic names
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}