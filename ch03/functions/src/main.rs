fn main() {
    println!("Hello, world!");
    another_function();
    with_a_parameter(5);
    multiple_parameters(6, 7);

    // Expressions & Statements

    // "let" is a statement and doesn't return a value.
    // let x = (let y = 6);
    
    let x = 5;
    let y = {
        let x = 3;
        x + 1  // Note: no semi-colon makes this an expression.
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
    println!("The value of x+1 is: {}", plus_one(x));
}

fn another_function() {
    println!("Another function.");
}

fn with_a_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn multiple_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {  // -> TYPE for return value
    5  // Note: No semi-colon for expression value to return!
}

fn plus_one(x: i32) -> i32 {
    // This doesn't work because Rust requires a trailing expression to return
    // a value from a function or an explicit "return" statement.
    // x + 1;
    x + 1
}
