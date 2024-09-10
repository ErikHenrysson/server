use std::fmt::{Display, Formatter, Result as FmtResult};


// Enumeration of all the implemented status codes
#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Ok = 200, 
    BadRequest = 400,
    NotFound = 404, 
}

impl StatusCode{
    // Function to return the correct message from the given StatusCode
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            
        }
    }
}

// Implementation of Display for StatusCode
impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}