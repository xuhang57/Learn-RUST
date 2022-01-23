// Validating References with Lifetimes
// Lifetime is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
// In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways

// Preventing Dangling References with Lifetimes

{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// The variable `x` does not "live long enough" as `x` will be out of scope when the inner scope ends on Line 13. But r is still valid for the outer scope
// To fix this, we need to use a Borrow Checker

fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}

// Generic Lifetimes in Functions

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Won't compile --> Rust cannot tell whether the reference being returned refers to x or y
// To fix this, we need to add generic lifetime parameters that defines the relationship between the references
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotation Syntax
&i32  // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime


// Lifetime Annotation in Function Signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str, // possible for structs to hold references, but we would need to add a lifetime annotation
}

fn main() {
    let novel = String::from("Call me Lucas. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a .");
    let i  = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime Elision
// The reason this function compiles without lifetime annotations is historical
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0...i];
        }
    }
    &s[..]
}

// would have written: fn first_word<'a>(s: &'a str) -> &'a str {}

// The first rule is that each parameter that is a reference gets it own lifetime parameter.
// In other words, a function with one parameter gets one lifetime parameter; a function with two parameters gets two separate lifetime parameters

// The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

// The third rule is tf there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters

// Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and _return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// The Static Lifetime
let s: &'static str = "I have a static lifetime"; // can live for the entire duration of the program. All string literals have the 'static lifetime

// Generic Type Parameters, Trait Bounds, and LIfetimes Together

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
