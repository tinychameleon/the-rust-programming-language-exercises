struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// A tuple struct. Has a name but no fields have names.
struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// "Unit" struct.. behaves kind of like the "Unit" type.
struct UnitStruct {}

fn print_user(user: &User) {
    println!("User<username: {}, email: {}, sign_in_count: {}, active: {}>",
             user.username, user.email, user.sign_in_count, user.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        // Long-hand form:
        // email: email,
        // username: username,
        // With identical names short-hand can be used:
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Structs can't be formatted by the default formatter.
    // println!("{}", user1);
    print_user(&user1);

    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);

    let userA = build_user(
        String::from("email@example.com"),
        String::from("theemailer"));
    print_user(&userA);

    // Update syntax for structs
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // Copying these from user1 is repetitive.
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        // Use the update syntax to take the unspecified values:
        ..user1
    };
    print_user(&user2);

    // Tuple structs
    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);
}
