use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{Request, Response, StatusCode, ParseError};

// Interface to handle good and bad requests
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest,None)
    }
}

// Structure of the server
// Holds a String representing the IP and port address
pub struct Server{
    addr: String,
}



impl Server{
    // Server Constructor
    pub fn new(addr: String) -> Self {
        return Self {
            addr
        }
    }
    
    // Function to run the TcpListener and handle the incoming requests and responses
    pub fn run(self, mut handler: impl Handler ) {
        
        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);
        
        loop {
            // Accepts incoming requests
            match listener.accept() {
                // Connection found
                Ok((mut stream, _)) => {
                    // Allocation of the response message buffer array
                    let mut buffer = [0; 1024];
                    // Adds the request to the buffer
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            // Deconstructs the the buffer String and creates a response to handle
                            let response= match Request::try_from(&buffer[..]) {
                                // The Response was created successfully
                                Ok(request) => handler.handle_request(&request),
                                // The Response was unsuccessful
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            // Sends the response to the end user
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }

                        },
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish connection: {}", e),
            }

        }
    }
}
