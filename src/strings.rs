// The String type is a Vec<u8> type that holds a valid UTF-8 sequence. 
// Strings are heap allocated, growable and not null terminated (whatever the last thing means).

// Meanwhile, an &str is a slice, an &[u8] type.

fn string_to_str() {
    let mut s: String = String::from("hello, world");

    // Use the 'as_str()' function to turn a String into a &str.
    let slice1: &str = s.as_str();
    assert_eq!(slice1, "hello, world");

    let slice2: &str = &s[..5];
    assert_eq!(slice2, "hello");

    // It's not entirely necessary to assign the variable in this way.
    // One could assign as thus:
    // let mut slice3: String = s;
    // Nonetheless, the current method of assignment would be helpful if extra code that
    // involved s would appear later on.
    let slice3: &mut String = &mut s;
    // Use push to append a single char onto the end of a current string.
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
}

fn iterate_thru_string() -> () {
    let s: String = String::from("hello, мир");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, 'м')
        }
    }
    println!("Success!");
}

fn utf8_to_string() -> () {
    let mut s: String = String::new();
    s.push_str("hello");

    let v: Vec<u8> = vec![104, 101, 108, 108, 111]; // hello
    // from_utf8 returns a Result so the result has to be unwrapped.
    let s1: String = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!");
}

/// Use with_capacity() to create a string in heap memory that has a predefined capacity.
/// Otherwise, String::new() will just allocate 0 and will dynamically expand based on
/// the size of new elements added to the string.
fn with_capacity() -> () {

    let mut s: String = String::with_capacity(25);

    println!("{}", s.capacity());
    
    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("{}", s);

    println!("Success!");
}