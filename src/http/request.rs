use super::method::{Method, MethodError};
use std::path::{self, Path};
use std::str::Utf8Error;
use core::{fmt, str};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8};
use super::{QueryString};


// Structure of the Request type. 
#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// Implementation of getters for the fields of the Request-type
impl<'buf> Request <'buf>{
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self)-> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// Implementation of the failable TryFrom function on the Request-type
// Takes a &str [u8] and returns a Result with the formated Request if Ok or an Error if not ok
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;
    
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;
        
        // Split the input into a method, path and protocol part
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        // The server implementation only supports HTTP/1.1 at the moment
        // TODO: implement other HTTP protocols
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;


        let mut query_string = None;

        // If there is a querystring, store it, otherwise only store the path
        // Querystring is optional in the request to the server
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path  = &path[..i];
        }
        
        // Return the formated request
        Ok(Self { 
            path,
            query_string: query_string,
            method: method 
        })
    }
}

// Help-function to deconstruct and split the original HTTP request string
fn get_next_word(request: &str) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    return None;
}

// Enumeration of all the possible errors while parsing the request.
pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

// Translation of ParseError enumeration to correct error-message.
impl ParseError{
    fn message(&self) -> &str{
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "invalid Method",
        }
    }
}

// Implementation of unfailable from<MethodError> method for Parserror
// Coverts a MethodError to a ParsError
impl From<MethodError> for ParseError{
    fn from(value: MethodError) -> Self {
        return Self::InvalidMethod;
    }
}

// Implementation of unfailable from<Utf8Error> method for ParserError
// Coverts Utf8Error to ParseError
impl From<Utf8Error> for ParseError{
    fn from(value: Utf8Error) -> Self {
        return Self::InvalidEncoding;
    }
}

// Implementation of Display method for ParseError
// Prints the ParseError message
impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Implementation of Debug for ParseError
// Prints the ParseError message
impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError{
    
}