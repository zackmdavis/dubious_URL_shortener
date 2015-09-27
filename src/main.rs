extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;


fn main() {
    Iron::new(|_req: &mut Request| {
        let destination = Url::parse(
            "https://github.com/rust-lang/cargo").unwrap();
        Ok(Response::with((status::Found, Redirect(destination))))
    }).http("localhost:64999").unwrap();
}
