fn main() {
    // Excerise #1: If Expressions (not a statement)
    // let number = 3;
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Conditions must be booleans.
    // if number {
    //    println!("number might be three");
    //}
    if number != 0 {
        println!("number is something other than zero");
    }

    // Multiple conditions through else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, o 2");
    }

    // If is an expression
    let number = if true { 5 } else { 6 };
    println!("The value of number is: {}", number);
    // Both blocks must have matching types
    // let number = if true { 5 } else { "6" };


    // Exercise #2: Loops (loop is an expression too)
    // loop {
    //     println!("again!");
    // }
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // "break" can return a value from the loop
        }
    };
    println!("The result is {}", result);

    // While loop over a condition
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // For loop over an iterator
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { // rev() reverses the range
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
