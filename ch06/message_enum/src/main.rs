#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // this is an anonymous struct!
    Write(String),
    ChangeColour(i32, i32, i32),
}

// Struct equivalents. But 4 different types.
//
// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColourMessage(i32, i32, i32);

// Enums can also have impl blocks.
impl Message {
    fn call(&self) {
        println!("Message: {:?}", self);
    }
}

fn main() {
    Message::Quit.call();
    Message::Move {x: 1, y: 2}.call();
    Message::Write(String::from("hello world")).call();
    Message::ChangeColour(10, 20, 30).call();
}
