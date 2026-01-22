fn main() {
    // lets return the longest of two strings
    let string1 = String::from("abcd");
    let string2 = "xyz";

    // life time of result, string1 and string2 all of same scope this is valid
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // this however is not valid as string2's lifetime is shorter than the result and string1
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");

    // lets use a struct with some lifetime generics
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    // we create this object with a str reference to a part of novel
    // the lifetime of the reference is valid and lasts as long as the obj so its allowed
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// the borrow checker does not know how the lifetime of x and y relate to the return type
// the references passed in pay have differing lifetimes, as such we dont know the lifetime of the return type
// we need to use lifetime generics here

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// Lifetime annotations don’t change how long any of the references live.
// Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
// Just as functions can accept any type when the signature specifies a generic type parameter,
// functions can accept references with any lifetime by specifying a generic lifetime parameter.
// this new lifetime generic tells the compiler that both parameters and the return type will live the same amount of time.
// were not changing these lifetime values, but rather telling the borrow checker to deny any inputs or outputs that void this signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// if we changed the function to only return one of the parameters, we would only care about that ones lifetime contract to the resulting type
fn longest_one_param<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// the return type lifetime must be related to the parameter lifetimes
// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//    let result = String::from("really long string");
//     result.as_str()
// }

// structs can also use generic lifetimes
// but in this case we would need to add a lifetime annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
// here we must declare lifetime of impl due to struct definition
impl<'a> ImportantExcerpt<'a> {
    // but due to first rule we can avoid annotating here
    fn level(&self) -> i32 {
        3
    }

    // third rule applies here and applies lifetime of self to all references here
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// 'static denotes that a lifetime can live for the entirety of a program
const stat: &'static str = "I have a static lifetime.";

// but wait how can this function compile without lifetime annotations despite using references
// in early versions of rust this code would not compile and all references would require a lifetime annotation
// the rust developers noticed many users repeating the same lifetimes in these deterministic situations
// as such they modified the compiler to include such patterns such that users did not have to annotate these anymore
// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lets pretend were the compiler and use the 3 lifetime inference rules to determine the lifetimes of the following function

// EX: 1
// the function starts of with no lifetimes
// fn first_word(s: &str) -> &str {}

// 1. The first rule applies lifetimes to all parameters
// fn first_word<'a>(s: &'a str) -> &str {}

// 2. The second rule states that if there is one parameter the output shares its lifetime
// fn first_word<'a>(s: &'a str) -> &'a str {}

// Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

// EX: 2
// start again with no lifetimes
// fn longest(x: &str, y: &str) -> &str {}

// 1. Apply lifetimes to all parameters
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

// 2. You can see that the second rule doesn’t apply because there is more than one input lifetime.
// 3. The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self
// RESULT: ERROR, After working through all three rules, we still haven’t figured out what the return type’s lifetime is.

// now lets put this chapter all together and use generics, traits and lifetimes
// below we have the longest with an announcement function
// it takes in two references and a generic T type that implements the Display trait
// because lifetimes are also generics we declare them in the same part of the function signature
// ann must implement Display because we print it out, and we must use lifetimes here due to not knowing the relationship
// between the input parameters and the output parameters lifetimes
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
