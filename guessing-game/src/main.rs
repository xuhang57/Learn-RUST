// io library in Rust, and it is from standard lib
use std::io;
use rand::Rng;

fn main() {
	println!("Guess the number!");
	
	// thread_rng() gives us a rand number that is local to the current thread of exec and seeded by the OS
	// inclusive on the lower bound, but exclusive on the upper bound
	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);

	println!("Please input your guess.");
    
	// let craetes a new variable and mut means the variable is mutable 
	// by deafult, a variable is immutable
	// String::new() creates a new String, utf-8 encoded bit of text
	// :: indicates that new is an associated function of the String type.
	// So new is a static method of String type
	let mut guess = String::new();

    // stdin is an associated function, stdin, on io. 
	// read_line takes whatever types into the std input and place it into a String
	// & means this argument is a reference, which gives you a way to let multiple parts
	// of your code access to one piece of data witout needing to copy that data into memory
	// multiple times. References are immutable by default so &mut guess makes it mutable
	// .epect is handling potential failure
	// read_line also returns a value: io::Result, which has types as enumerations(enum)
	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");

    // {} is place holder for the value in guess
    println!("You guessed: {}", guess);
}

// By default, Rust brings only a few types into the scope of every program in the prelude
// 
