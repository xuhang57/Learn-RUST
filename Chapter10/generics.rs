// Generic Data Types

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// This won't work as largest won't work for all possible type of `T` could be. We can only use types whose values can be ordered!
// To enable comparisons, the std library has the `std::cmp::PartialOrd` trait that you can implement on types
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics in Struct Definitions

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
}

struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In Method Definitions

Struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}

// We could implement methods only on Point<f32> instances rather than on Point<T>
impl Point<f32> {
    fn distance_from_orgin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// A method that uses different generic types from its struct's definition
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello", y: 'c'};

    let p3  = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


