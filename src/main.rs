extern crate iron;

use iron::prelude::*;
use iron::status;


fn main() {
    Iron::new(|_req: &mut Request| {
        Ok(Response::with((status::Ok, "Hello, \"World\"!\n")))
    }).http("localhost:64999").unwrap();
}
