fn main() {
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuples can't be printed by default.
    // println!("A tuple: {}", tup);
    
    // Destructuring Tuples
    // You can ignore values and provide a name by prefixing with _
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // Indexing from 0 also works.
    println!("The value of x is: {}", tup.0);
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);

    // Arrays
    let _a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // [3, 3, 3, 3, 3]

    println!("The value of a[2] is {}", a[2]);
    println!("The value of b[4] is {}", b[4]);

    // Out of bounds access panics at runtime and can be detected by the
    // compiler sometimes.
    // println!("The value of a[100] is {}", a[100]);
}
