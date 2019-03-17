// io library in Rust, and it is from standard lib
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // thread_rng() gives us a rand number that is local to the current thread of exec and seeded by the OS
    // inclusive on the lower bound, but exclusive on the upper bound
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // for debugging purpose
    println!("The secret number is: {}", secret_number);

    // loop keyword creates an infinite loop
    loop {
        println!("Please input your guess.");

        // let creates a new variable and mut means the variable is mutable 
        // by default, a variable is immutable
        // String::new() creates a new String, utf-8 encoded bit of text
        // :: indicates that new is an associated function of the String type.
        // So new is a static method of String type
        let mut guess = String::new();

        // stdin is an associated function, stdin, on io. 
        // read_line takes whatever types into the std input and place it into a String
        // & means this argument is a reference, which gives you a way to let multiple parts
        // of your code access to one piece of data without needing to copy that data into memory
        // multiple times. References are immutable by default so &mut guess makes it mutable
        // .expect is handling potential failure
        // read_line also returns a value: io::Result, which has types as enumerations(enum)
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // i32, a 32-bit number; u32, an unsigned 32-bit number, i64, u64
        // The colon(:) after guess tells Rust we'll annotate the variable type.
        // The parse() could easily cause an error, and it returns "Result" type
        // much as the read_line() method does
        // rather than crashing the program when the user inputs a non-number, let's
        // make the game ignore a non-number so the user can continue guessing (String => u32)
        // using a match expression
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} is place holder for the value in guess
        println!("You guessed: {}", guess);

        // Ordering is another enum, but the variants for Ordering is Less, Greater, Equal
        // match expression is made up of arms. An arm consists of a pattern and the code
        // that should be run if the value given to the beginning of the match expression
        // fits that arm's pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// By default, Rust brings only a few types into the scope of every program in the prelude
// If a type you want to use isn't in the preclude, you have to bring that type into scope explicitly
// with a use statement.

// cargo doc --open , which will build documentation provided by all of your dependencies locally
// and open it in your browser.
