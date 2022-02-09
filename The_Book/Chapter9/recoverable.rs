// Most errors are not serious enough to require the program to stop entirely

enum Result<T, E> {
    Ok(T), // T represents the type of the value that will be returned in a success case within the ok variable
    Err(E),  // E represents the type of the error that will be returned in a failure case within the Err variable
}

use std::fs::File;
use std::fs::{self, Read};
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}, error");
    }

    let b = File::open("hello.txt");

    let b = match b {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}, e"),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            Fill::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // The Result<T, E> type has many helper methods defined on it to do various tasks. One method is called unwrap
    // which is a shortcut method that is implemented just like the match expression
    let f = File::open("hello.txt").unwrap(); // unwrap will return the value inside the ok, and panic! if Err

    // Another shortcut is called expect, which is similar to unwrap, lets us also choose the panic! error message
    let f = File::open("hello.txt").expect("Failed to open");
}


// Propagating Errors
// fn read_username_from_file() -> Result<String, io:Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// A Shortcut for Propagating Errors: the ? Operator
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt");
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // no ; means return
}

// The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values
// The difference is that the errors values that have the ? operator called on them go through the from function, defined in the From trait in std

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok((())
}

// Box<dyn Error> type is called a trait object, which will be discussed in Chapter 17 --> any kind of error
