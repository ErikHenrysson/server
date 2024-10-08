use std::{fmt::{Display, Formatter, Result as FmtResult}, net::TcpStream};
use super::StatusCode;
use std::io::{Write, Result as IoResult};

// Structure of the Response to the request
// Contains a statuscode and an optional body to return
#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}



impl Response{
    // Response constructor
    // Takes a statuscode and an optional body and returns a new Response
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Response{
            status_code, body
        }
    }

    // Function to send data to the given stream
    // Takes a stream that has an implementation of the Write function and returns an IoResult
    // Writes directly to the stream.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }
}
