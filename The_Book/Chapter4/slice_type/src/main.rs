fn main() {
    let mut s = String::from("hello wolrd");

    let word = first_word(&s);

    // Since String literals are string slices already, this works too
    // let my_string_literal = "hello word";
    // let word = first_word(my_string_literal)

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, bu there is no more string that
    // we could meaningfully use the value 5 with, word is now totally invalid!

    // borrow later. Borrowing rules that if we have an immutable reference to something
    // we cannot also take a mutable reference
    // because clear() needs to truncate the String, it tries to make a mutable reference, which fails
    println!("the first word is: {}", word);

    let hello = &s[0..5];
    let world = &s[6..11];

    // let hello = &s[0..=4];
    // let hello = &s[..5];
    // let world = &s[6..=10];
}
// returns a byte index value into the String parameter
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     // enumerate() method returns a tuple, we can use patterns to destructure that tuple
//     // we use & in the pattern because we get a pointer from the iter().enumerate() 
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// get back a single value that is tied to the underlying data. The value is made up of a reference
// to the starting point of the slice and the number of elements in the slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // enumerate() method returns a tuple, we can use patterns to destructure that tuple
    // we use & in the pattern because we get a pointer from the iter().enumerate() 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}



/*
Slice doesn't not have ownership. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection

The concepts of ownership, borrowing, and slices ensure memory safety in Rust program at compile time. Ownership affects how lots of other parts
of Rust work.
*/