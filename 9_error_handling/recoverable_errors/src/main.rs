use std::{fs::{self, File}, io::{Error, ErrorKind, Read}};

// here is an example of us using the Result value
// in the case that the file correctly opens we get a return value of
// Ok(File_handle)
// however if this operation were to fail we would get the return value of
// Err(e)
fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file_result2 = File::open("hello.txt");

    // due to the unknown nature of the return type we must handle both cases
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // we can also match based on different errors found
    let greeting_files = match greeting_file_result2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // using 'unwrap' we will either get back the value Ok() or the program will panic all within one line
    let greeting_file = File::open("hello.txt").unwrap();

    // we can also use expect to convey some message about what went wrong
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

// lets use an example of error propagation
fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// we can use the ? operator as a error propagation shortcut
// this function has the same logic as the one above but is more readable
// instead of matching every error we can simply use ?
// another interesting thing about the ? operator is that is calls From
// therefore we could use some cutom error type in the result and it would convert to it
// for this to work we would need to define impl From<io::Error> for our custom error
fn read_username_from_file_2() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// here is an even more shortened version from method chaining
fn read_username_from_file_3() -> Result<String, Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// and an even shorter approach using an already implemented helper
// because returing result for reading files is so common its been implemented already
fn read_username_from_file_4() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

// heres an example of where using the ? operator would not work
// ? Can only be used on type Result, Option, or another type that implements FromResidual.
// fn main() {
//     let greeting_file = File::open("hello.txt")?;
// }

// heres an example of using the ? operator with type Optional
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
