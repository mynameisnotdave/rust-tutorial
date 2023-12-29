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