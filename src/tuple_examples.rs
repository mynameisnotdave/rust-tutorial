fn main() -> () {
    // This allows myself to understand that tuples can hold multiple datatypes of any amount.
    // Additionally, instead of printing 'Success!' I decided to print the values of the tuple.
    // This allowed me to understand the importance of the semicolon and the wildcard symbol inside
    // the curly parantheses; this allows the tuple to print.
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8, (i16, u32)) = (0, (-1, 1));

    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("{:?}", t);
}