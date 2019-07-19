extern crate rustc_serialize;
use rustc_serialize::json;

extern crate nickel;
use nickel::{Request, Response, MiddlewareResult};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname:  String,
}

pub fn get_person<'r, 'mw, 'conn>(_: &mut Request<'mw, 'conn>, response: Response<'mw>) -> MiddlewareResult<'mw> {
    let person = Person{
        firstname: String::from("Bob"),
        lastname: String::from("Dylan"),
    };
    let json = json::encode(&person).unwrap();
    response.send(json)
}

pub fn hello(word: &str) -> String
{
    String::from(format!("Hello {}", word))
}