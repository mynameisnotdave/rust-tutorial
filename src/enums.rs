enum IpAddr {
    V4(String),
    V6(String),
}

fn assign_enums() ->() {
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
}

enum Number {
    Zero = 0,
    One = 1,
    Two = 2
}

fn cast_enums_as_ints() -> () {
    // This shows how to use enums as integers by casting them with the 'as' keyword.
    assert_ne!(Number::One as u8, Number::Two as u8);
}

// This shows how enums can hold different types of data.
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// This shows how to instantiate those different data types.
fn message_impl() -> () {
    let msg1 = Message::Move { x: (3), y: (4) };
    let msg2 = Message::Write(String::from("hello world"));
}

// This shows how to destructure a value in an enum type in an 'if let' statement.
fn destructure_enum_val() -> () {
    let msg: Message = Message::Move{x: 1, y: 1};

    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a,b);
    } else {
        panic!("NEVER LET THIS RUN! ");
    }
}

fn enum_array() -> () {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: (1), y: (3) },
        Message::ChangeColor(255, 255, 0)
    ];

    for msg in msgs {
        // This appears to show the importance of semicolons in Rust.
        // Note that the semi colon is omitted here.
        // Presumably, if there was a semicolon, then the foreach loop
        // would terminate early.
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}