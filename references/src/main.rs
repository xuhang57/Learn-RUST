fn main() {
    let s1 = String::from("hello");

    // &s1 creates a reference that refers to the value of s1 but does not own it.
    let len = calculate_length(&s1);

    println!("The length of '{}' is '{}'", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s is: '{}'", s);

    // cannot do this since only one mutable reference to a particular piece of data in a particular scope
    // let r1 = &mut s;
    // let r2 = &mut s;
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problem
    let r2 = &mut s;

    // let reference_to_nothing = dangle();
}

// define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership
// of the value
fn calculate_length(s: &String) -> usize {
    // since we are borrowing, we cannot modify it. 
    // For example, we cannot do s.put_str(", world");
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// liftimes. 
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// } // Here, s goes out of scope, and is dropped. Its memory goes away so it is dangerous

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


/*
       s                    s1
| name | value |      | name | value |        | index | value |
| ptr  |   ----|----> | ptr  |  -----|------> |   0   |   h   |
                      | len  |   5   |        |   1   |   e   |
                      | cap. |   5   |        |   2   |   l   |
                                              |   3   |   l   |
                                              |   5   |   o   |

referencing: &
dereferencing: *

reference cannot be modified, unless we add a keyword mut
mutable references have one big restriction: you can have only one mutable reference to a particular piece of data
in a particular scope.
The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to
a race condition and happens when these three behaviors occur
    Two or more pointers access the same data at the same time
    At least one of the pointers is being used to write to the data
    There's no mechanism being used to synchronize access to the data
Data races cause undefined behavior and can be difficult to diagnose and fix when you are trying to track them down at
runtime; Rust prevents this problem from happening because it won't even compile code with data races. 

We also cannot have a mutable reference while we have an immutable one.
Dangling pointer, a pointer that references a location in memory that may have been given to someone else, by freeing some
memory while preserving a pointer to that memory. 
In Rust, the compiler guarantee that references will never be dangling references: if you have a reference to some data,
the compiler will ensure that the data will not go out of scope before the reference to the data does.
*/
