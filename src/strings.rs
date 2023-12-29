// The String type is a Vec<u8> type that holds a valid UTF-8 sequence. 
// Strings are heap allocated, growable and not null terminated (whatever the last thing means).

// Meanwhile, an &str is a slice, an &[u8] type.