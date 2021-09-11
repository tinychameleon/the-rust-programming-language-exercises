// String is in the stdlib, not the core language! Only string slice (str)
// is part of the core language (normally seen as &str).

fn main() {
    // As with other types, there is an associated function new()
    let mut s = String::new();
    println!("s = {:?}", s);

    // Initial data can come from any object that implements Display
    // which has a method "to_string()".
    let data = "initial contents";
    s = data.to_string(); // can also be called directly on the literal.
    println!("s = \"{}\"", s); // String implements Display!

    // The from() associated method can create a String from a str
    let s: String = String::from("another string");
    println!("{}", s);

    // All strings in Rust are UTF-8 encoded.
    let s: String = String::from("Здравствуйте");
    println!("{}", s);

    // Mutable strings can be extended via push_str.
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // There is also push() to extend via a single character.
    s.push('!');
    println!("{}", s);

    // Concatenation is a little tricky and seems far more complicated
    // than it should be.
    let s1 = String::from("Hello, ");
    let s2 = "world!".to_string();
    // let s3 = s1 + s2;  <- wrong. s2 expected to be &str
    // let s3 = &s1 + s2; <- wrong. can't concatenate &str with String.
    // let s3 = &s1 + &s2; <- wrong. can't concatenate two &str values. 
    let s3 = s1 + &s2; // right. + modifies s1 with a copy of s2.
                       // there is also implicit coercion occuring to turn
                       // &s2 from a &String into a &str.
                       // Essentially &s2 is treated like &s2[..]
    println!("{}", s3);

    // It's pretty gnarly. s1 needs to be moved, and the others need refs.
    let s1 = "tic".to_string();
    let s2 = "tac".to_string();
    let s3 = "toe".to_string();
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // There is a format! macro to paper over this which operates like println!
    // but returns a String. Similar to things like fmt.Sprint in Go.
    let s1 = "tic".to_string();
    let s = format!("{}-{}-{}", s1, s2, s3); // No moves occur.
    println!("{}", s);

    // Rust strings are UTF-8 and do not allow indexing. Since 1 to 4 bytes
    // represent each character it is not guaranteed to get a valid script
    // character back. Rust simply disallows this action.
    //
    // Rust does allow you to slice a chunk of a string out. If you get the
    // encoded byte sizes for a script character wrong Rust will panic.
    let hello = "Здравствуйте".to_string();
    // let s = &hello[0]; <- can't grab a single byte via indexing.
    let s = &hello[0..2]; // can grab the script character if you know the size
    // let s = &hello[0..1]; // this panics. "З" is not 3. It is 2 bytes long.
    println!("{} {}", hello, s);

    // If you need to iterate over the bytes of a string use bytes().
    for b in "Здравствуйте".bytes() {
        print!("{} ", b);
    }
    println!();

    // If you need to iterate over script characters use chars().
    for c in "Здравствуйте".chars() {
        print!("{} ", c);
    }
    println!();
}
