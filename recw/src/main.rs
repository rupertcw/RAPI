extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MediaType, MiddlewareResult};
use nickel::hyper::method::Method;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname:  String,
}

fn get_person<'r, 'mw, 'conn>(_: &mut Request<'mw, 'conn>, response: Response<'mw>) -> MiddlewareResult<'mw> {
    let person = Person{
        firstname: String::from("Bob"),
        lastname: String::from("Dylan"),
    };
    let json = json::encode(&person).unwrap();
    response.send(json)
}

fn hello(word: &str) -> String
{
    String::from(format!("Hello {}", word))
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", middleware!({format!("{}", hello("index"))}));
    server.get("/hello", middleware!({format!("{}", hello("world"))}));
    server.add_route(Method::Get, "/person", get_person);
    server.listen("127.0.0.1:6767");
}