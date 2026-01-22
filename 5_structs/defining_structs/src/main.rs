struct User {
    is_active: bool,
    username: String,
    password: String,
    login_count: u64,
}

// Tuple Struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit Struct
struct AlwaysEqual;

fn main() {
    print!("Structs!");

    let mut user_one = User {
        is_active: true,
        username: String::from("Default User"),
        password: String::from("Default Password"),
        login_count: 0,
    };

    user_one.login_count = 1;

    let new_user = create_user(true, String::from("MyUser"), String::from("My Password"), 1);

    // Can copy attributes from another user using this cool .. shorthand
    // Does not appear to take ownership looks like it does a copy
    let user2 = User {
        username: String::from("another_name"),
        ..new_user
    };

    print!("{}", new_user.login_count);

    // Init Tuple Struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

// Function to create and return a user
fn create_user(
    new_active: bool,
    new_user: String,
    new_password: String,
    new_login_count: u64,
) -> User {
    User {
        is_active: new_active,
        username: new_user,
        password: new_password,
        login_count: new_login_count,
    }
}

// Short hand creation method
fn create_user_two(password: String, username: String) -> User {
    User {
        is_active: true,
        username,
        password,
        login_count: 1,
    }
}
