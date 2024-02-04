use std::num::ParseIntError;

// Error handling. The second value of the generic is a string because the error returns
// a string message.
fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("Division by zero");
    }
    Ok(x / y)
}

fn do_the_divide() -> () {
    let result = divide(10.0, 2.0);

    // This does something with the result based upon the arguments given inside the
    // result-bound function.
    match result {
        Ok(val) => println!("Result, {}", val),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn something_or_parseinterror() -> Result<(), ParseIntError> {
    let number_str = "10";
    // the ? is a shorthand match for any result functions.
    // it tells the compiler that this variable might be associated with an error result.
    let number = number_str.parse::<i32>()?; 
    println!("{}", number);
    Ok(())
}