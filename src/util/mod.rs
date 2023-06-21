use std::error::Error;

pub mod error;
impl error {
    
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
