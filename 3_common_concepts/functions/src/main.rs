fn main() {
    println!("Hello, world!");

    another_function(20,'h');

    /* use of an expression */
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    /* call a function as an expression */
    let five: i32 = five();
}

fn another_function(value: u32, unit_label: char) {
    println!("Result is {value}{unit_label}");
}

fn five() -> i32 {
    5 /* <- if a ';' is added here it becomes a statement and gives a compile error */
}