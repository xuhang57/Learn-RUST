fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31); // "31 days"

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // Alice, this is Bob. Bob, this is Alice

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over"); // the quick brown fox jumps over the lazy dog

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2); // 1 of 10 people know binary, the other half doesn't

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond"); // error

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3)); // error
}

// std::fmt contains many traits which govern the display of text. The base form of two important ones are listed below:

// fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
// fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
