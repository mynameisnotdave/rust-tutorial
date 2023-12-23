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