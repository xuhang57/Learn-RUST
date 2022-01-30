# Error Handling

In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.

Rust groups errors into two major categories:

* recoverable
* unrecoverable

Rust does not have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.

## To panic! or Not to panic!

When code panics, there is no way to recover. You could call `panic!` for any error situation, whether there is a possible way to recover or not, but then you are making the decision
on behalf of the code calling your code that a situation is unrecoverable. When you choose to return a `Result` value, you give the calling code options rather than making the decision
for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one.

## Cases in Which You Have More Information Than the Compiler

It would also be appropriate to call `unwrap` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic is not something the compiler understands

```rust
use std::net::IpAddr;

let home: ipAddr = "127.0.0.1".parse().unwrap();
```
k
## Guidelines for Error Handling

* The bad state is not something that is expected to happen occasionally
* Your code after this point needs to rely on not being in this bad state
* There is not a good way to encode this information in the types you use

## Creating Custom Types for Validation

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
