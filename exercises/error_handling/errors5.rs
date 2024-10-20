use std::error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct CreationError;

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Creation error")
    }
}

impl error::Error for CreationError {}

fn main() -> Result<(), Box<dyn error::Error>> {
    let number_str = "42";
    let number: i32 = number_str.parse()?;

    if number == 0 {
        Err(Box::new(CreationError))
    } else {
        println!("Number is valid: {}", number);
        Ok(())
    }
}
