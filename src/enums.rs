enum IpAddr {
    V4(String),
    V6(String),
}

fn assign_enums() ->() {
    let home: IpAddr = IpAddr::V4(String::from("127.0.0.1"));

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
}