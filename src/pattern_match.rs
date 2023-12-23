enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    // In a match statement, every case has to be handled.
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn if_let() -> () {
    let config_max: Option<u8> = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
// We can turn the above into....

fn simplified_if_let() -> () {
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max); 
    }
}

enum Direction {
    East,
    West,
    North,
    South
}

// Using the discard is basically saying, 'any other value, do this.'
fn match_direction() -> () {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}

// 'Match is an expression, so we can use it in assignments.'
fn assign_with_match() -> () {
    let boolean: bool = true;

    let binary: i32 = match boolean {
        true => 1,
        false => 0
    };
}