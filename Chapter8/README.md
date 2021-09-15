# Common Collections

Collections are data structures. Most other data types represent one specific value, but collections can contain multiple values. Unlike the builtin array and tuple types,
the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs

* A vector allows you to store a variable number of values next to each other
* A string is a collection of characters.
* A hashmap allows you to associate a value with a particular key.

[More Details](https://doc.rust-lang.org/std/collections/index.html)

## Storing Lists of Values with Vectors

### Creating a New Vector

```rust
let v: Vec<i32> = Vec::new();
```

Vectors are implemented using generics. Vec<T>. In more realistic code, Rust can often infer the type of value you want to store once you insert values.

```rust
let v = vec![1, 2, 3];  // Create a vector within values
```

### Updating a Vector

```rust
let mut v= Vec::new();

v.push(1);
v.push(2);
v.push(3);
```

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope

```rust
{
    let v = vec![1,2,3];
    // do stuff with v
} // <- v goes out of scope and is freed
```

### Reading Elements of Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

let thrid: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element");
}
```

Rust has two ways to reference an element so you can choose how the program behaves when you try to use an index value that the vector does not have an element for.

```rust
let v = vec![1,2,3,4,5];

let does_not_exist = &v[100];     // 1
let does_not_exist = v.get(100);  // 2
```

The first method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there is
an attempt to access an element past the end of vector.

The second method will return `None` without panicking. You would use this method if accessing an element beyond the range of the vector happens occasionally under normal situation.

### Iterating over the Values in a Vector

```rust
let v = vec![100, 101, 120];
for i in &v {
    println!("{}", i);
}
```

We can also iterate over mutable references to each element in mutable vector in order to make changes to all the elements

```rust
let mut v = vec![12, 21, 33];
for i in &mut v {
    *i += 50;
}
```

To change the value that the mutable reference refers to, we have to use the dereference operator (*) to get to the value in `i` before we can use the `+=` opertaor.

### Use an Enum to Store Multiple Types

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage
is that we can be explicit about what types are allowed in this vector.

Using an `Enum` plus a `match` expression means that Rust will ensure at compile time that every possible case is handled. If you don't know the exhaustive set of types the program
will get at runtime, the enum technique won't work, you can use a trait object

## Storing UTF-8 Encoded Text with Strings
