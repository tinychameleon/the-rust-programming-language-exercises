fn main() {
    // Example #1: Mutability
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Example #2: Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum point value is: {}", MAX_POINTS);

    // Example #3: Shadowing & Immutable Transformations
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Example #4: Shadowing & Changing Variable Types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);

    // Example #5: Mutable Variables Can't Change Type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
