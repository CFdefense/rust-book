fn main() {
    /* References allow us to pass values without changing ownership */
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    /* Mutable reference usage */
    let mut s = String::from("hello");

    change(&mut s);

    /* Can use another scope to make multiple mutable references but not two simultaneous mut refs */
    let mut t = String::from("hello");

    {
        let r1 = &mut t;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut t;

    /*
        Cannot have a mutable and immutable reference to the same value this will error
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // BIG PROBLEM

        println!("{}, {}, and {}", r1, r2, r3);
    */

    /*
        The scopes of the immutable references r1 and r2 end after the println!
        where they are last used, which is before the mutable reference r3 is created.
        These scopes don’t overlap, so this code is allowed:
        the compiler can tell that the reference is no longer being used at a point
        before the end of the scope.
    */

    let mut c = String::from("hello");

    let r1 = &c; // no problem
    let r2 = &c; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut c; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
