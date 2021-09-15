# Managing Growing Projects with Packages, Crates, and Modules

## Overview

For very large projects of a set of interrelated packages that evolve together, Cargo provides workspaces, which would be covered in Chapter 14

In addition to grouping functionality, encapsulating implementation details lets you reuse code at a higher level: once you have implemented an operation, other
code can call that code via the code's public interface without knowing how the implementation works.

A related concept is scope: the nested context in which code is written has a set of names that are defined as "in scope". When reading, writing, and compiling
code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant or other
item and what that item means.

## Module System

* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function or a module

## Packages and Crates

A crate is a binary or library. The crate root is a source file that Rust compiler starts from and makes up the root module of your crate.
A package is one or more crates that provide a set of functionality.
A package contains a Cargo.toml file that describes how to build those crates

For example: rand::Rng

## Defining Modules to Control Scope and Privacy

The `use` keyword brings a path into scope; and the `pub` keyword make items public.

`Modules` let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can
be reused by outside code (public) or is an internal implementation detail and not available for outside use (private)

## Paths for Referring to an Item in the Module Tree

A path can take two forms:

1. An absolute path starts from a crate root by using a crate name or a literal `crate`
2. A relative path starts from the current module and use `self`, `super`, or an identifier in the current module

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

```rust
mod front_of_host {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Modules are not useful only for organizing your code, they also define Rust's `privacy boundary`: the line that encapsulates the implementation detail external code is not
allowed to know about, call, or rely on.

The way privacy works in Rust is that all items (functions, methods, structs, enums, and constants) are private by default. Items in a parent module cannot use the private
items inside child modules. But items in chile modules can use the items in their ancestor modules.

### Starting Relative Paths with super

`super` is like starting a filesystem path with the `..` syntax.

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order;
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,       // public
        seasonal_fruit: String,  // private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the sumer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change the mind
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // won't compile
    mea.seasonal_fruit = String::from("blueberry");
}
```

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
        Bread,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
}
```

## Bringing Paths into Scope with the use Keyword

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // use self::front_of_house::hosting

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist;
}
```

Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.

### Creating Idiomatic use Paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_hose::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

### Providing New Names with the as Keyword

```rust
use std::fmt::Result as fmtResult
use std::io::Result as IoResult
```

### Re-exporting names with pub use

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

By using `pub use`, external code can now call the `add_to_waitlist` function using `hosting::add_to_waitlist`.
Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.
With `pub use`, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers
working on the library and programmers calling the library

### Using External Packages

In Cargo.toml, add `rand = "0.8.3"`.

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}
```

### Using Nested Paths to Clean Up Large use Lists

```rust
use std::cmp::Ordering;
use std::io;

use std::io;
use std::io::Write;
```

```rust
use std::{cmp::Ordering, io};

use std::io::{self, Write};
```

## Separating Modules into Different Files

```rust
mod front_of_house;

pub mod hosting;
```

Using a semicolon after `mod front_of_host` rather than using a block tells Rust to load the contents of the module from another file within the same name as the module
