use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            };
        } 
        
        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_preference1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_preference1);

    println!("User1 of preference {:?} is awarded the shirt color {:?}", user_preference1, giveaway1);

    let user_preference2 = None;
    let giveaway2 = store.giveaway(user_preference2);

    println!("User2 of preference {:?} is awarded the shirt color {:?}", user_preference2, giveaway2);

    // random closure example with type anno
        let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // going from fn to anon closure
    fn  add_one_v1   (x: u32) -> u32                      { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32|        { x + 1 };
    let add_one_v4 = |x: u32|          x + 1  ;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello")); // compiler infers this type
    // let n = example_closure(5);                          // therefore this is inferred to need a string and will give error

    // closures will implicitly decide whether or no to take ownership/borrow mutably/immutably depedending on body

    // this first example because we only use the value we borrow immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");

    only_borrows(); // if we store the closure in a variable we can call it normally
    println!("After calling closure: {list:?}");

    // however in this second example we are modifying a mutable vector so the closure borrows mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7); // mutable change

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // we can force the closure to take ownership with 'move' this is very useful for threads
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // Lets now look at how the sort_by_key fn, it implements the trait FnMut
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value); // fails here as it tries to move value
        r.width
    });
    println!("{list:#?}");

    // instead we will use a mutable counter variable and use FnMut correctly
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");

}

/* 
    Lets analyze the unwrap_or_else method used before
    We see that it implmenets FnOnce meaning the function must and will be called once.
    We also see that it returns type T
    We also see that in the body of the fn, we only call the passed fn if x is None
        impl<T> Option<T> {
            pub fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T
            {
                match self {
                    Some(x) => x,
                    None => f(),
                }
            }
        }
*/