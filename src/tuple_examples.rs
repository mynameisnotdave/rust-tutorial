fn tuple_types() -> () {
    // This allows myself to understand that tuples can hold multiple datatypes of any amount.
    // Additionally, instead of printing 'Success!' I decided to print the values of the tuple.
    // This allowed me to understand the importance of the semicolon and the wildcard symbol inside
    // the curly parantheses; this allows the tuple to print.
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8, (i16, u32)) = (0, (-1, 1));

    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("{:?}", t);
}

fn tuple_references() -> () {
    // Understanding how to access data inside tuples by reference.
    // Understanding that the index of a tuple in Rust starts from 0.
    let t: (&str, &str, &str) = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
}

fn tuple_too_long () -> () {
    // This would not work. I understand that tuples over 12 elements long are not printable.
    let too_long_tuple: (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long of a tuple: {:?}", too_long_tuple);
}

fn destructure_tuple () -> () {
    // This demonstrates how to destructure a tuple. This means assignment of the tuple's values
    // to individual variables for each value in the tuple.
    let tup: (i32, f64, &str) = (1, 6.4, "hello");
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}

fn assign_tuples_later() -> () {
    // This shows how to destructure assignments, in other words, creating a tuple with no values,
    // just assignments, and then assigning values to each variable later on.
    let (x, y, z);

    (y, z, x) = (1, 2, 3);
}