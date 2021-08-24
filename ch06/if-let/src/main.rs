fn main() {
    // Normal exhaustive matching, we need to match against every possibility.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }

    // The if-let construct allows matching on just one possibility.
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
