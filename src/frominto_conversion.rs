use std::fs;
use std::io;
use std::num;

#[derive(Debug)]
struct Number {
    value: i32
}

impl From<i32> for Number {
    fn from(n: i32) -> Number {
        Number {
            value: n
        }
    }
}
 
fn use_conversion() -> () {
    let num: Number = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}


// This is how to implement custom error types. The open_and_parse_file function utilises the
// CliError type which is an enum containing the IoError and ParseError types. These need
// to be converted into something tangible for the Rust compiler to understand when the
// error type inside the Result of the open_and_parse_file function is CliError.

// It uses the From trait to understand that io::Error or num::ParseIntError can 
//'belong' and be used by the CliError type.
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError)
}

impl From<io::Error> for CliError {
    fn from(e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(e: num::ParseIntError) -> Self {
        CliError::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let contents: String = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    return Ok(num)
}