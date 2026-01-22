fn main() {
    /* Assignment Using Boolean Expressions */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /* endless repetition 
    loop {
        println!("again!");
    }
    */

    /* Declare Using Loops */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    /* Breaking out of specific loops when a condition is met */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    /* Using 'for' to iterate through a collection */
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    /* We can also use ranges and reverse it */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
