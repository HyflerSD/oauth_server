pub mod generator;
use hyper::{body::Body, Request, Response};
use std::convert::Infallible;

pub fn run() {
    println!("greetings!");
}


pub async fn handle_request(req: Request<dyn Body>) -> Result<Response<dyn Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/somepath") => {
            Ok(Response::new(Body::from(String::from("Success"))))
        }
        (&hyper::Method::POST, "/create") => {
            Ok(Response::new(Body::from(String::from("Success Create "))))
        }

        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Body::from(String::from("bad")))
                .unwrap())
        }
    }
}
