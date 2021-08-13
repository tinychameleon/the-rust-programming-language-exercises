fn main() {
    // References borrow; no ownership is transfered with &s1 below.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Mutating a reference does not work. They are immutable by default like
    // variables. Mutable references require the referred variable to ALSO be
    // mutable.
    let mut s = String::from("hello");
    change(&mut s);
    // let s = String::from("hello");
    // change(&s);
    println!("s = {}", s);

    // Only ONE mutable reference can be in scope at any time. Simultaneous
    // mutation is not allowed. This prevents data races through references.
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    // Creating a new scope for mutable reference is ok.
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    // Mutable and immutable references within the same scope is NOT allowed.
    // Code with an immutable reference expects the referred to value to not
    // change. Allowing a mutable reference at the same time would destroy
    // that guarantee.
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);

    // Reference scope continues from the point it is introduced to the last
    // place it is used.
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // r1 and r2 scope ends.

    let r3 = &mut s; // no immutable references in scope now.
    println!("{}", r3);

    // The compiler ensures referenced data lives longer than the reference.
    // No dangling pointers :)
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
// fn change(some_string: &String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // reference to something that will no longer exist.
} // s goes out of scope here and Drop frees the memory for it.
