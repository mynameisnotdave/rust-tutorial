fn lifetime() -> () {
    let r;
    // This is a good example of a so-called 'lifetime'. r is defined w/o a value. Then, inside this scope,
    // x = 5, and then a reference of x is defined to r.
    // x has then gone out of scope. But the value tries to live on, and this is why things fail.
    // x does not live long enough, in comparison to r.
    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// Lifetimes as function arguments look like generics. The lifetime value is annotated with a single quote mark.
// The lifetime exists because the value must live longer than this function.
fn print_one<'a>(x: &'a i32) -> () {
    println!("`print_one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) -> () {
    *x += 1;
}

// Ensure that the correct lifetime has been returned.
fn pass_x<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    y
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn call_longest() -> () {
    let x: &str = "long";
    let y: &str = "longer";
}