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

let third: &i32 = &v[2];
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

New Rustaceans commonly get stuck on strings for a combination of three reasons:

* Rust's propensity for exposing possible errors
* strings being a more complicated data structure than many programmers give them credit for
* UTF-8

### What is a String?

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`. The `String` type, which is provided by Rust
Standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

### Creating a new String

```rust
let mut s = String::new();
```

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly
let s = "initial contents".to_string()
```

```rust
let s = String::from("initial contents");
```

### Updating a String

A `String` can grow in size and its content can change, just like the contents of a `Vec<T>`, if you push more data into i.

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

The `push` method takes a single character as a parameter and adds it to the `String`.

```rust
let mut s = String::from("lo");
s.push('l');
```

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used.
```

The reason `s1` is no longer valid after the addition and the reason we used a reference to `s2` has to do with teh signature of the method that gets called
when we use `+` operator. The `+` operator uses the `add` method, whose signature looks something like:

```rust
fn add(self, s: &str) -> String {}
```

First, `s2` has an `&`, which means that we are adding a reference of the second string to the first string because of the `s` parameter in the add function:
we can only add a `&str` to a String; we cannot add two `String` values together. The reason we are able to use `&s2` in the call to add is that the compiler
can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns `&s2` into `&s2[..]`.

Second, we can see in the signature that add takes ownership of self, because self does not have an `&`. This means `s1` will be moved into the add call and no
longer be valid after that. So although `let s3 = s1 + &s2;`, looks like it will copy both strings and create a new one, this statement actually takes ownership
of `s1`, appends a copy of the contents of `s2`, and then returns ownership of the result.

If we need to concatenate multiple strings, the behavior of the `+` operator gets unwieldy:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3; // s = "tic-tac-toe"
```

To simplify, we could have:

```rust

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### Indexing into Strings

In many other languages, accessing individual characters in a string by referencing them by index is a valid and common operations. However, if you try to access
parts of a `String` using indexing syntax in Rust, you will get an error. In other words, Rust does not support indexings for Strings.

### Internal Representation

A String is a wrapper over a `Vec<u8>`.

```rust
let hello = String::from("Hola");
```

In this case, `len` will be 4, which means the vector storing the string "Hola" is 4 bytes long. Each of those letters takes 1 byte when encoding in UTF-8. However if
it is other languages suc has Cyrillic letters, it takes more bytes to encode a character because each Unicode scalar value in strings takes 2 bytes of storages. Therefore
an index into the string's bytes will not always correlate to a valid Unicode scalar value.

### Slicing Strings

Indexing into a string is often a bad idea because it is not clear what the return type of string-indexing should be: a byte value, a character, a grapheme cluster, or a string
slice. Rust asks you to be more specific if you really need to use indices to create string slices.

```rust
let hello = "ABCD";
let s = &hello[0..4];
```

Here `s` will be a &str that contains the first 4 bytes of the string.

### Methods for Iterating Over Strings

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}

for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

## Storing Keys with Associated Values in Hash Maps

Hash maps are useful when you want to look up data not by using an index, as you can with vector, but by using a key that can be of any type.

### Creating a New Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

The type annotation `HashMap<_, _>` is needed here because it is possible to `collect` into many different data structure and Rust does not know
which you want unless you specify. With underscores, Rust can infer the type of the data in the vectors.


### Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// feild_name and field_value are invalid at this point
```

If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

### Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name); // returns Some(&10)
```

We can also iterate over each key/value pair in a hash map in a similar manner as we do with vectors:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}, {}", key, value);
}
```

### Overwriting a value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 50);

println("{:?}", scores); // {"Blue", 25}
```

### Only inserting a value if the key has no value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(60);
scores.entry(String::from("Blue")).or_insert(50);

println("{:?}", scores);
```

### Updating a value based on the old value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}, map); // {"world": 2, "hello": 1, "wonderful": 1}
```

The `or_insert` method actually returns a mutable reference &mut V to the value of its key. Here we store that mutable reference in the `count` variable, so in order to assgin to that value, we must first dereference `count` with asterisk (`*`). The mutable reference goes out of scope at the end of the `for` loop, so all of these changes are safe and allowed by the borrowing rules.

### Hash Functions

By default, `HashMap` uses a hashing function called SipHash that can provide resistance to Denial of Service attacks involving hash tables. Not the fastest but the trade-off for better security.
