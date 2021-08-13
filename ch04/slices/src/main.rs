fn main() {
    // Without slices, how do you interact with parts of a String?
    // By index, but the index is not directly related to the String
    // itself.
    let s = String::from("Hello World");
    println!("First word in 'Hello World' ends at {}", first_word_index(&s));

    // Slices are a reference to a range within a String.
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    // but also ranges can be missing bounds
    println!("{} {} {}", &s[..5], &s[6..], &s[..]);

    // First word again, but as a slice.
    let s = String::from("hello world");
    println!("First word in 'hello world' is: {}", first_word(&s));
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
