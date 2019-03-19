fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    let x = five();
    println!("In main, the value of x is: {}", x);

    let x = plus_one();
    println!("after plus_one, the value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };
    println!("In main, the value of y is: {}", y);
}

fn another_function(x: i32, y: i32) {
    println!("Another Function!");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // no ":" since this function needs to return a value. With having the ":", it is not returning
    x + 1
}

/*
Rust uses snake case as the conventional style for function and variable names. All letters are lowercase
and underscores separate words. 
*/