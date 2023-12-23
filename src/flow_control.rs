fn if_else() -> () {
    // Basic if-else functionality in Rust.
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}


// Variables can be assigned by adding conditions to a previous variable.
fn assign_by_conditional() -> () {
    let n: i32 = 5;

    let big_n: i32 =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            // This cannot have a semi colon as it would terminate the if-else
            // statement prematurely.
            10 * n 
        } else {
            println!(", and is a big number, halve the number");

            n / 2.0;
        };
    println!("{} -> {}", n, big_n);
}