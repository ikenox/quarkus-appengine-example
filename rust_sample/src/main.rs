extern crate iron;
 
use iron::prelude::*;
use iron::status;
use std::env;

fn main() {
let default_port=8080;
        let port = match env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => default_port
        },
        Err(_) => default_port,
    };

    println!("listen on port:{}", port);

    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello Rust world!")))
    }).http(format!("localhost:{}",port)).unwrap();
}
