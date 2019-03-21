fn main() {
    /*
    let number = 3;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }
    */

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    
    assert_eq!(result, 20);

    // let mut num = 3;

    // while num != 0 {
    //     println!("{}!", num);
    //     num = num -1;
    // }
    // println!("LIFTOFF!");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     // This could cause the program panic if the index length is incorrect. It is also slow, because 
    //     // the compiler adds runtime code to perform the conditional check on every element on every iteration
    //     // through the loop
    //     println!("the value is: {}", a[index]);
    //     index = index + 1;
    // }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


}
