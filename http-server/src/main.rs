extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::{Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

/// We need to return different futures depending on the route matched,
/// and we can do that with an enum, such as `futures::Either`, or with
/// trait objects.
///
/// A boxed Future (trait object) is used as it is easier to understand
/// and extend with more types. Advanced users could switch to `Either`.
type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
fn echo(req: Request<Body>) -> BoxFut {
	let mut response = Response::new(Body::empty());

	match (req.method(), req.uri().path()) {
		// Serve some instructions at /
		(&Method::GET, "/") => {
			*response.body_mut() = Body::from("Hello World");
		}

		// Simply echo the body back to the client.
		(&Method::GET, "/test") => {
			*response.body_mut() = req.into_body();
		}

		// The 404 Not Found route...
		_ => {
			*response.status_mut() = StatusCode::NOT_FOUND;
		}
	};

	Box::new(future::ok(response))
}

fn main() {
	let addr = ([127, 0, 0, 1], 3000).into();

	let server = Server::bind(&addr)
		.serve(|| service_fn(echo))
		.map_err(|e| eprintln!("server error: {}", e));

	println!("Listening on http://{}", addr);
	hyper::rt::run(server);
}