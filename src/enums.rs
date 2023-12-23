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