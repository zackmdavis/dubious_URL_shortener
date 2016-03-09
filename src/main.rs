#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen)]

#[macro_use] extern crate diesel;
extern crate pencil;
extern crate hyper;

pub mod schema;
pub mod models;

use std::env;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use pencil::{Pencil, Request, Response, PencilResult};
use hyper::header::Headers;


fn establish_database_connection() -> SqliteConnection {
     let our_database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&our_database_url)
        .expect(&format!("Regrettably, an error whlie attempting \
                          to connect to {}", our_database_url))
}


fn uniformly_locate_resource(_our_client_request: &mut Request) -> PencilResult {
    let mut our_server_response = Response::new_empty();
    our_server_response.status_code = 302;
    let mut our_headers = Headers::new();
    our_headers.set_raw("Location", vec![
        b"https://github.com/fengsp/pencil".to_vec()]);
    our_server_response.headers = our_headers;
    Ok(our_server_response)
}


fn main() {
    let mut app = Pencil::new("/");
    app.get("/<supplied_resource_identifier>",
            "uniformly_locate_resource", uniformly_locate_resource);
    app.run("localhost:64999");
}
