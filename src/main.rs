extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;
use iron::headers::Location;

fn main() {
    Iron::new(|req: &mut Request| {
	let mut path = String::new();
	for piece in req.url.path.iter() {
		path.push_str(piece);
		path.push_str("/");
	}
	path.push_str("hehe");

        Ok(Response::with((status::SeeOther, Header(Location(path)))))
    }).http("0.0.0.0:3000").unwrap();
}
