
use gui::{Draw, Button, Screen};

fn main() {
    // define the screen and its subsequent components
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // call run on the screen to draw components
    screen.run();
}

struct SelectBox {
    // struct representing a gui select box
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

mod gui {
    pub trait Draw {
        // Our Draw trait requires those implementing it to implement draw
        fn draw(&self);
    }

    pub struct Screen {
        // Screen contains a vector of trait objects that must impl Draw
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            // run method will call draw on each trait object in components
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    pub struct Button {
        // struct representing a gui button
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for Button {
        // implements override of draw method from trait Draw
        fn draw(&self) {
            // code to actually draw a button
        }
    }
}