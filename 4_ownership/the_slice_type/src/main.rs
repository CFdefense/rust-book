fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    /* String slice example */
    let s = String::from("hello world");

    let hello = &s[0..5]; /* 0..5 == ..5 */
    let world = &s[6..11]; /* we could use 6..len or 6.. instead */

    /*
        Now we can correctly call the first word method and maintain our data integrity despite changes to t
        We will correctly get a compile error now
    let mut t = String::from("hello world");

    let word = new_first_word(&t);

    t.clear(); // error!

    println!("the first word is: {word}");

    */

    /* String literals are stored as &str, this explains why literals are */
    let s = "Hello, world!";

    /* Other examples of slices */
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

/* Lets try to return a subset of a string (ie the first word) without using a slice */
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn new_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    /* Returns the entire thing as a slice */
    &s[..]
}
