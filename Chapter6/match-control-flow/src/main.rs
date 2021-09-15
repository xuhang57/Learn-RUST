/* Match is a control flow operator that allows you to compare a value against a
series of patterns and then execute code based on which pattern matches.

Patterns can be made up of literal values, variable names, wildcards, and many
other things.
*/

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Patterns that bind to Values

#[derive(Debug)] // inspect the state
enum UsState {
    Alabama,
    Alaska,
    // --skip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska))

// Matching with Option<T>

fn plus_one(x:Option<i32>) -> Option<i32> {
    match x { 
        None => None,
        Some(i) => Some(i+1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three");
    _ => (),
}

// concise control flow with if let
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
// ==
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}

let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    // _ placeholder: matches any value
    _ => count += 1,
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else { // same as the block of code that would go with the _ case in the match expression
    count += 1;
}

fn main() {
    println!("Hello, world!");
}

/*
Combining match and enums is useful in many situations. You will see this pattern a lot in Rust code:
match against an enum, bind a variable to the data inside, and then execute code based on it.

Matches are exhaustive. Rust knows that we did not cover every possible case and even knows which pattern
we forgot. We must exhaust every last possibility in order for the code to be valid.

The _ pattern will match any value. It's called a placeholder.

However, the match expression can be a bit wordy in a situation in which we care about only one of the cases.
We could just use `if let`.

You can think of `if let` as syntax sugar for a match that runs code when the value matches one pattern
and then ignores all other values. You would lose the exhaustive checking that match enforces.

Choosing between match and if let depends on what you are doing in your particular situation and whether
gaining conciseness is an appropriate trade-off losing exhaustive checking

Creating custom types to use in your API ensures type safety: the compiler will make certain your functions
get only values of the type each function expects

*/