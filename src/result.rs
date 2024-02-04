use std::num::ParseIntError;

use crate::result;

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


fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
    let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();

    Ok(n1.unwrap() * n2.unwrap())
}

fn do_the_multiply() -> () {
    let result: Result<i32, ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4","2");
    assert_eq!(result, Ok(8));

    println!("Success!");
}