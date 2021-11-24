fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding); // inner short 2
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding); // outer long 1
}

// Variable shadowing is allowed
fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}

// Outputs

// before being shadowed: 1
// shadowed in inner block: abc
// outside inner block: 1
// shadowed in outer block: 2
