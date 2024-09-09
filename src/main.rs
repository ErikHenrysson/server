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
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path = {}", public_path);
    let addr = "127.0.0.1:8080";
    let server = Server::new(addr.to_string());
    server.run(WebsiteHandler::new(public_path));
}



