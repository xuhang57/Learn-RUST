// It's possible to declare variable bindings first, and initialize them later.
// However, this form is seldom used, as it may lead to the use of uninitialized variables.


fn main() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding); // a binding: 4

    let another_binding;

    // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding); // another binding: 1
}
