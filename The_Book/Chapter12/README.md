# An I/O Project: Building a Command Line Program

Rust's speed, safely, single binary output, and cross-platform support make it an ideal language for creating command line tools.

This project is our own version of the classic command line tool: `grep` (globally search a regular expression and print).

Our project will combine a number of concepts you've learned so far:

* Organizing code (Chapter 7)
* Using vectors and strings (Chapter 8)
* Handling errors (Chapter 9)
* Using traits and lifetimes where appropriate (Chapter 10)
* Writing tests (Chapter 11)

## Refactoring to Improve Modularity and Error Handling

* Split your program into a main.rs and a `lib.rs` and move your program’s logic to lib.rs
* As long as your command line parsing logic is small, it can remain in `main.rs`
* When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`

The responsibilities that remain in the main function after this process should be limited to the following:

* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a run function in lib.rs
* Handling the error if run returns an error

`main.rs` handles running the program, and `lib.rs` handles all the logic of the task at han. Because you cannot test the `main` function directly, this structure lets you test all of your program's logic by moving it into functions in `lib.rs`. The only coed that remains in `main.rs` will be small enough to verify its correctness by reading it.