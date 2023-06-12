enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Received Quit message");
        }
        Message::Move { x, y } => {
            println!("Move to coordinates ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("Write message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    let msg1 = Message::Quit;
    process_message(msg1);

    let msg2 = Message::Move { x: 10, y: 20 };
    process_message(msg2);

    let msg3 = Message::Write(String::from("Hello, Rust!"));
    process_message(msg3);

    let msg4 = Message::ChangeColor(255, 128, 0);
    process_message(msg4);
}
