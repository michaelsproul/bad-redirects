extern crate iron;
extern crate mount;

use iron::prelude::*;
use iron::status::Status;
use iron::modifiers::Header;
use iron::headers::Location;
use mount::{Mount, OriginalUrl};

// Simple macro to generate handler closures.
macro_rules! handle {
    ($code:expr) => {
        |req: &mut Request| { handle_redirect($code, req) }
    }
}

// Status-code generic redirect handler.
// Adds "hehe" to the end of the current URL path and returns a response with
// the Location header set to this new path.
fn handle_redirect(status_code: Status, req: &mut Request) -> IronResult<Response> {
    let mut path = String::new();

    // Add the top-level part of the URL (the redirect number).
    let orig_url = req.extensions.get::<OriginalUrl>().unwrap();

    path.push_str("/");
    path.push_str(&orig_url.path[0]);
    path.push_str("/");

    // Add all the pieces of the redirect path so far.
    for piece in req.url.path.iter() {
        path.push_str(piece);
        path.push_str("/");
    }

    // Add another piece of redirect path.
    path.push_str("hehe");

    Ok(Response::with((status_code, Header(Location(path)))))
}

fn main() {
    use iron::status::Status::*;

    let mut mount = Mount::new();
    mount.mount("/300/", handle!(MultipleChoices));
    mount.mount("/301/", handle!(MovedPermanently));
    mount.mount("/302/", handle!(Found));
    mount.mount("/303/", handle!(SeeOther));
    mount.mount("/304/", handle!(NotModified));
    mount.mount("/305/", handle!(UseProxy));
    mount.mount("/307/", handle!(TemporaryRedirect));
    mount.mount("/308/", handle!(PermanentRedirect));

    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
