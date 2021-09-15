/* Enums allow you define a type by enumerating its possible variants. Rust's
enums are most similar to algebraic data types in functional languages.
*/

// a custom data type that we can use elsewhere in the code
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// more concise way
enum IpAddr {
    V4(String),
    V6(String),
}

// let home = IpAddr::V4(String::from("127.0.0.01"));

enum IpAddr1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct Ipv4Addr {}
// struct Ipv6Addr {}
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum Message {
    Quit,
    Moev {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32) // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("Hello"));
m.call();

// The Option Enum and its advantages over null values
enum Option<T> {
    Some<T>,
    None,
}

let some_number = Some(5);
let some_string = Some("a string");

// When using None, we need to tell Rust what type of Option<T> we have
let absent_number: Option<i32> = None;

// following lines will fail
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x + y; // no implementation for `i8 + Option<i8>`

/*
Each variant in Enums can have different types and amounts of associated data

Rust does not have the null feature that many other language have. Null is a value
that means there is no value there. In language with null, variable can always
be in one of two states: null or not-null.

The problem with null values is that if you try to use a null value as a not-null
value, you'll get an error of some kind. Because this null or not-null property is
pervasive, it's extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null
is a value that is currently invalid or absent for some reason.

Rust does not have nulls, but it does have an enum that can encode the concept of a
value being present or absent. This enum is Option<T>

*/
