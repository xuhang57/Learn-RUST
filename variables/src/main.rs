/*
 * Including: Variables and Mutability, Data Types in Chap 3
 */

fn main() {
    /*
    let x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable x
    x = 6;
    println!("The value of x is: {}", x);
    */

    let mut x = 5;
    println!("The value of x is: {}", x);
    // cannot assign twice to immutable variable x
    x = 6;
    println!("The value of x is: {}", x);

    // You cannot use mut with const and the type of the value must be annotated
    // Constants can be declared in any scope, including the global scope
    // Rust's naming convention for constants is to use all uppercase with underscores
    // between words, and underscores can be inserted in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000;

    // Shadowing a variable: y
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing can change the variable type
    let spaces = " ";
    let spaces = spaces.len();

    /*
    // but we cannot do this
    let mut spaces = " ";
    spaces = spaces.len();
    */

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    // Booleans are one byte in size
    let t = true;

    // explicit type annotation
    let f: bool = false;

    // Unicode Scalar Value
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = "😻";
    println!("heart_eyed_cat: {}", heart_eyed_cat);


    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {}", b);

    let five_hundred = a.0;
    let six_point_four = x.1;
    let one = x.2;

    let arr = [1,2,3,4,5];
    // fixed size: 5
    let arr: [i32; 5] = [1,2,3,4,5];
    
    let first = a[0];
    let second = a[1];
}

/*
 * As we know, by default variables are immutable. When a variable is immutable,
 * once a value is bound to a name, you cannot change that value.
 * In Rust, the compiler guarantees that when you state that a value won't change,
 * it really won't change. Use mut keyword in front of the variable name to make 
 * the variable immutable.
 */

/*
There are multiple trade-offs consider in addition to the prevention of bugs. For example,
in cases where you are using large data structures, mutating an instance in place may be
faster than copying and returning newly allocated instances. With smaller data structures,
creating new instances and writing in a more functional programming style may be easier to
think through, so lower performance might be a worthwhile penalty for gaining that clarity
*/

/*
Constants are valid for the entire time a program runs, within the score they were declared in,
making them a useful choice for values in your app domain that multiple parts of the program
might need to know about
*/

/*
Shadowing is different than marking a variable as mut, because we will get a compile-time error
if we accidentally try to re-assign to this variable without using the let keyword. Shadowing is
creating a new variable when we use the let keyword again.
*/

/*
Rust is a statically typed language, which means it must know the types of all variables at compile
time. A scalar type represents a single value. Rush has four(4) primary scalar types: integers,
floating-point numbers, Booleans and characters. 
*/

/*
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples
and arrays. A tuple is a general way of grouping together some number of other values with a variety
of types into one compound type. Tuple has a fixed length: once declared, they cannot grow or shrink in size
Array, unlike tuple, must have the same type element and it has a fixed length as well. 
Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to
ensure you always have a fixed number of elements. For example, 12 months in a year, etc.
*/