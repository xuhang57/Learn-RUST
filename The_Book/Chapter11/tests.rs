// A test in Rust is a function that's annotated with the test attribute. Attributes are metadata about pieces of Rust code.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
} // -> cargo test

// Check results with the assert! Marco
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
}

// TEsting Equality with the assert_eq! and assert_ne! Marcos

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// Adding Custom Failure Messages
pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_2() {
        let result = greeting("Lucas");
        assert!(
            result.contains("Lucas"),
            "Greeting does not contain name, value was `{}`",
            result
        );
    }
}

// Checking for Panics with should_panic
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}",, value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100 {
        Guess::new(200);
    }
}

// Using Result<T, E> in Tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 = 4 {
            Ok(())
        } else {
            Err(String::from("wrong"))
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

#[test]
#[ignore]
fn it_works() {
    assert_eq!(2+2, 4);
}
