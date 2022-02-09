# Writing Automated Tests

Three actions:

1. Setup any needed data or state
2. Run the code you want to test
3. Assert results are what you expect

## Test Organization

The Rust community thinks about tests in terms of two main categories:

* Unit Tests
* Integration Tests

## Unit Tests

Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.

The purpose of unit test is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn't working as expected. You'll put unit tests in the `src` directory in each file with the code they are testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

### The Tests Module and $[cfg(test)]

The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
```

### Testing Private Functions

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests

Integration tests are entirely external to you library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test

In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library's public API.

### The tests Directory

```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

### Submodules in Integration Tests

```rust
// tests/common.rs

pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

```rust
//tests/integration_test_rs

use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```