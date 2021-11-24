fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer); // An integer: 1
    println!("A boolean: {:?}", a_boolean);  // A boolean: true
    println!("Meet the unit value: {:?}", unit); // Meet the unit value: ()

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;
    // let _noisy_unused_variable = 2u32;
}