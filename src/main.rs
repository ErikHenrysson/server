#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod server;
mod http;

use server::Server;
use http::Request;
use http::Method;

fn main() {
    let addr = "127.0.0.1:8080";
    let server = Server::new(addr.to_string());
    server.run();
}



