use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

//Structure of the WebsiteHandler
pub struct WebsiteHandler{
    public_path:String
}


impl WebsiteHandler {
    // Constructor for the WebsiteHandler
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }
    // Function to read a file at a given path in the public folder
    // Takes a &str file path as input and returns an optional string containing the body of the response
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        
        // Match over the canonicalized path (Removes any attempts of attacks)
        match fs::canonicalize(path) {
            Ok(path) => {
                // Read the file if the given path leads to a file in the public folder 
                if path.starts_with(fs::canonicalize(&self.public_path).unwrap()){
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted to Path: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

// Implementation of the Handler interface for the WebsiteHandler
// The default implementation of handle_bad_request is ok for this application
impl Handler for WebsiteHandler {
    // Function to handle a good request
    // Takes a formated Request and returns a Response 
    fn handle_request(&mut self, request: &Request) -> Response {
        // Match over the possible methods
        // TODO: Implement other methods than GET
        match request.method() {
            Method::GET => match request.path() {
                // Handle the default request
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),

                // Handle request with specified path other than "/"
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            // If the path was not found 
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}