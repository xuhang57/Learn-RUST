fn main() {
    // String type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    let mut s = String::from("hello");

    // push_str appends a literal to a String
    s.push_str(", world!");
    println!("{}", s);

    // Won't be able to do this since s1 is no longer valid
    // this is called a "move" in Rust
    // let s1 = String:from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    // deep copy using clone() in heap
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // stack copy since int is known size type and we use stack to store them
    // If a type has the Copy trait, an older variable is still usable after assignment
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let c = String::from("hello");
    // c's value moves into the function, so no longer valid here
    takes_ownership(c);

    let w = 5;
    // w would move into the function, so it is okay to use w afterward
    makes_copy(w);

    // gives_ownership moves its return value into s1
    let c1 = gives_ownership();
    let c2 = String::from("hello");
    // c2 is moved into takes_and_gives_back, which also moves its return value into c3
    let c3 = takes_and_gives_back(s2);

    let t1 = String::from("hello");
    let(s2, len) = calculate_length(t1);
    println!("The length of '{} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns length of a String

    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and "drop" is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}




/*
All programs have to manage the way they use a computer's memory while running. 
Some languages have garbage collection that constantly looks for no longer used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free memory. Rust uses a third approach: memory is managed through
a system of ownership with a set of rules that the compiler checks at compile time.

In a systems programming language like Rust, whether a value is on the stack or the heap
has more of an effect on how the language behaves and why you have to make certain decisions

Both the stack and the heap are parts of memory that are available to your code to use
at runtime, but they are structured in different ways. The stack implements a LIFO. It is
fast because of the way it accesses the data: it never has to search for a place to put
new data or a place to get data from because that place is always the top. Fixed size also
makes stack fast.

Data with a size unknown at compile time or a size might change can be stored on the heap
instead. The heap is less organized: when you put data on the heap, you ask for some amount
of space. The OS finds an empty spot somewhere in the heap that is big enough, marks it as
being in user and returns a pointer, which is the address of that location. (Allocating on the heap)
Accessing data in the heap is slower than accessing data on the stack because you have to follow
a pointer to get there. 

When your code calls a function, the values passed into the function(including, potentially,
pointers to data on the heap) and the function's local variables get pushed onto stack. When
the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount
of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of
space are all problems that ownership addresses.

Ownership Rules
1. Each value in Rust has a variable that is called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped.

The data type are all stored on the stack and popped off the stack when there scope is over,
but we want ot look at the data that is stored on the heap and explore how Rust knows when to
clean up that data.

In the case of a string liter, we know the contents at compile time, so the text is hardcoded
directly into the final executable. That's why it is fast and efficient. But it is immutability
While String type, in order to support a mutable, growable piece of text, we need to allocate an
amount of memory on he heap, unknown at compile time, to hold the contents

In Rust, the memory is automatically returned once the variable that owns it goes out of scope.
Basically Rust calls drop automatically at the closing curly bracket.

By design, Rust will never automatically create "deep" copies of your data

The semantics for passing a value to a function are similar to those for assigning a value to a variable
Passing a variable to a function will move or copy, just as assignment does

Returning values can also transfer ownership.

Ownership is a key feature in Rust, so read more: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
*/
