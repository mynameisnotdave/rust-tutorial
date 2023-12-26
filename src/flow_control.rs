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

fn break_for() -> () {
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

// This shows how to use the 'loop' loop in Rust. This creates an infinite loop that must
// be deliberately broken out of.
fn break_loop() -> () {
    let mut count: u32 = 0;

    loop {
        count += 1;
        
        if count == 3 {
            println!("three");

            break;
        }
        
        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }
}

// This shows how to assign a value using a loop.
fn assign_loop_to_val() -> () {
    let mut count: i32 = 0;

    let result: i32 = loop {
        count += 1;
        
        if count == 10 {
            break count * 2;
        }
    }; // if a value is assigned via a loop, then a semicolon must be added

    assert_eq!(result, 20);

    println!("Success!");
}

// This shows how to use inner and outer loops. They must be annotated. 
fn outer_and_inner_loops() -> () {
    let mut count: i32 = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }

            continue 'outer;
        }
    }

    assert!(count == 30);

    println!("Success!");
}