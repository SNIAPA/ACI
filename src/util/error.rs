use core::fmt;
use std::error::Error;


#[derive(Debug, Clone)]
pub struct CheatError{
    message: String
}


impl Error for CheatError{}

impl fmt::Display for CheatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cheat Error: {}", self.message)
    }
}

impl CheatError {
    pub fn new(message: &str) -> CheatError{
        CheatError {message:message.to_owned()}
    }
}
