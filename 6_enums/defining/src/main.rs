enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrTypeTwo {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddrTypeTwo::V4(127,0,0,1);
    let loopback = IpAddrTypeTwo::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    
    // Some i32 number or none (Inferred)
    let some_number = Some(5);

    // Some char or none (Inferred)
    let some_char = Some('e');

    // Some i32 or none (Inferred)
    let absent_number: Option<i32> = None;

    /* 
    Will not compile cant add to a possible None value
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    */

}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


