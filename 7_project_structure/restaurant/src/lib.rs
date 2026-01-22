mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// we can also bring modules into scope with 'use'
use crate::front_of_house::hosting;

// we can also do this to change the imported names
use crate::back_of_house::Appetizer as AppetizerFromBack;

// we can also 're-export' imported modules to make them available for others to import
// pub use crate::customer::eat_at_restaurant;

// we can also use public packages
use rand::Rng;

// we can do nested paths aswell
// use std::cmp::Ordering;
// use std::io;
// becomes
// use std::{cmp::Ordering, io};

// and also like this
// use std::io;
// use std::io::Write;
// becomes
// use std::io::{self, Write};

// or use all with
// use std::collections::*;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

mod customer {
    pub fn eat_at_restaurant() {
        // inside we cannot use the 'use' because we are now outside its scope
        //hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // lets order
    let mut meal = back_of_house::Breakfast::summer("Rye Bread");

    // lets change our order
    meal.toast = String::from("White Bread");

    // we cant modify or see the private member of seasonal fruit
    // meal.seasonal_fruit = String::from("Strawberries");

    // pub enum fields are all pub by default
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


    // we can call the 'use' item like this
    hosting::add_to_waitlist();
}

fn deliver_order() {}


mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // use 'cd ..' to parent module then use its method
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // the user doesnt get to pick this the chef does
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("Peaches") 
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}