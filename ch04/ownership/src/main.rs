fn main() {
    // Simple values are "Copy"
    let x = 5;
    let y = x;
    println!("x = {} and y = {}", x, y);

    // Complex values may not be "Copy". String is not. It gets moved.
    let s1 = String::from("hello");
    // let s2 = s1; // Moves the value of s1 into s2, leaving s1 unusable.
    let s2 = s1.clone();
    println!("s1 = '{}' and s2 = '{}'", s1, s2);

    // Function calls are the same. Their arguments get moved or copied.
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // s has been moved.
    let x = 5;
    makes_copy(x);
    println!("x is still {}", x);

    // Function return values also can move or copy.
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
