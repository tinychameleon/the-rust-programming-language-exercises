#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // No elements mean the type needs to be specified.
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    // Use the vec! macro to create a vector without type specifications.
    // I peaked behind the curtain and the macro uses "<[_]>::into_vec()".
    // Pretty sure it's a type annotation of "any slice" and the into_vec method.
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    // Updating a vector can use push. I'm sure there are many methods on Vec.
    let mut v3: Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    println!("{:?}", v3);

    // Vectors drop their elements upon being dropped themselves. Ownership.
    
    // Reading values from a vector. Type annotations are added for clarity.
    // Using [] panics when the index is out of range. get() does not.
    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2];
    println!("The third element is {}", third);
    match v4.get(2) { // get returns an Option<&T> so third is still a &i32
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over a vector. For loops will move vectors because Vec is not Copy.
    // Need to use references.
    let v5 = vec![100, 32, 57];
    let mut v6 = vec![100, 32, 57];

    for i in &v5 {
        println!("{}", i);
    }

    for i in &mut v6 {
        *i += 50;
    }

    println!("{:?}", v5);
    println!("{:?}", v6);

    // Since Vec can only hold one type, enums are a good mechanism to store multiple
    // variants of a single type.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
