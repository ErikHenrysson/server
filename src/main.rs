#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod server;
mod http;
mod website_handler;
use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;


fn main() {
    // Gets the path to the public folder
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    // Takes another, specified path as an input from the user
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path = {}", public_path);
    // Specifies the IP and port that the TcpListener should listen to
    let addr = "127.0.0.1:8080";
    let server = Server::new(addr.to_string());
    server.run(WebsiteHandler::new(public_path));
}



