fn main() {

    /* Cannot Assign Twice to a IMMUTABLE VARIABL: Will NOT Work
        let x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The Value of x is: {x}");
    */

    /* If we make x 'mutable' this will not work and be reassigned */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* Despite including calculations, rust can determine these before runtime and this will compile */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /* Use of 'Shadowing (Notice how no 'mut' is used) */
    let x = 5; // init to 5

    let x = x + 1; // rewritten to 6

    {
        let x = x * 2; // used previous 6 and multiplies it by two
        println!("The value of x in the inner scope is: {x}"); // within this scope x is 12
    }

    println!("The value of x is: {x}"); // once we leave scope the shadow is gone and x is 6 again


    /* We are not allowed to mutate this mutable variables' type
        let mut spaces = "   ";
        spaces = spaces.len();
    */

    /* However we can use shadowing to acheive the same effect */
    let spaces = "   ";
    let spaces = spaces.len();
}
