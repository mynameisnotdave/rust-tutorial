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

fn enumerate() -> () {
    let a: [i32; 4] = [4, 3, 2, 1];

    // Here we have index i and v representing a. The iter().enumerate() code
    // iterates through the array.
    for(i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i+1,v);
    }
}

fn break_loop() -> () {
    let mut n: i32 = 0;
    // Formatting this as 0..=100 means that 100 will be included.
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");
}