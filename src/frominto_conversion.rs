use std::fmt::Result;
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
    // The question marks below is to associate that result with a particular error.
    // In this case, the contents variable will be associated with the IoError, 
    // and the num variable will be associated with the ParseError.
    let contents: String = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    return Ok(num)
}

fn try_from_into() -> () {
    let n: i16 = 256;

    // This is a way to catch type conversions that may fail.
    // Instead of confidently using the 'into' method, we have a 'try into' which will
    // provide a way for us to error check the conversion gracefully.
    // In this case, this conversion will not work. It will throw an error as 256 is
    // too big for i16.
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("caught error trying to convert {:?}", e.to_string());
            0
        }
    };
    assert_eq!(n, 0);
    println!("Success!")
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

// This is a way to test for even numbers using the try_from function.
impl TryFrom<i32> for EvenNum {
    type Error = ();

    fn try_from(value: i32) -> Result {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        }
        else {
            Err(())
       }
    }
}

fn even_num() -> () {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!");
}