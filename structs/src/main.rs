/* 
A struct (structure) is a custom data type that lets you name and package together multiple related values
that make up a meaningful group. A struct is like an object's data attribute in OOP. Using structs and enums
are the building blocks for creating new types in your program's domain to take full advantage of Rust's
compile time type checking. 

*/

// adding the annotation to derive the Debug trait and printing the Rectangle instance using debug formatting
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// defining a method
impl Rectangle {
    // self is Rectangle due to this method's being inside impl Rectangle context
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    // fields
    username: String,
    // ownership String rather than &str string slice type
    // so the instances of this struct to own all of its data and for that data to be valid for
    // as long as the entire struct is valid
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email;
        email,
        //username: username;
        username,
        active: true,
        sign_in_count: 1,
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
   rectangle.width * rectangle.height
}


fn main() {
    // in instance of the User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    // if mutable (entire instance), we can change
    user1.email = String::from("another@example.com");

    // struct update syntax, .. means that the remaining fields not explicityly set should have
    // the same value as the fields in the given instance
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser123"),
        // active: user1.active;
        // sign_in_count: user1.sign_in_count,
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of a rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // let rect1 = (30, 50);

    // println!(
    //     "The area of a rectangle is {} square pixels.",
    //     area(rect1)
    // );

    let rect1 = Rectangle { width: 30, height: 50};

    println!(
        "The area of a rectangle is {} square pixels.",
        area(&rect1)
    );

    /* pirntln!("rect1 is {}", rec1) won't work
       println! uses formatting known as Display: output intended for direct end user consumption.
       println!(:?) for debug
    */
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area (called method) of a rectangle is {} square pixels.",
        rect1.area()
    );

    
}


/*
Rust doesn't allow us to mark only certain fields as mutable. The entire instance has to be mutable

tuple structs have the added meaning the struct name provides but don't have names associated with 
their fields; rather, they just have the types of the fields. They are useful when you want to give 
the whole tuple a name and make the tuple be a different type than other tuples, and naming each field
as in a regular struct would be verbose or redundant 

unit-like structs are structs that don't have any fields, similar to "()", the unit type
They are useful in situations in which you need to implement a trait on some type but don't have any data
that you want to store in the type itself

It is possible for structs to store references to data owned by something else, but to do so requires the
use of lifetimes, a Rust feature that ensures the data referenced by a struct is valid for as long as the
struct is. 

Methods are similar to functions: they are declared with the fn keyword and their name, they can have parameters
and a return value, and they contain some code tht is run when they are called from somewhere else.
The main benefit using method is to put all the things we ca ndo with an instance of a type in one impl block
rather than making future users of our code search for capabilities of Rectangle in various places

"->"

".": in C/C++, two different operators are used for calling methods, one is "."
*/

