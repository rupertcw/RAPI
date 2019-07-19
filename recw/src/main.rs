#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};
use nickel::hyper::method::Method;

mod routes;


fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!({format!("{}", routes::hello("index"))}));
    server.get("/hello", middleware!({format!("{}", routes::hello("world"))}));
    server.add_route(Method::Get, "/person", routes::get_person);
    match server.listen("127.0.0.1:6767") {
        Ok(_res) => {println!("Server has started")}
        Err(e) => {println!("Server failed to start: {:?}", e)}
    }
}

