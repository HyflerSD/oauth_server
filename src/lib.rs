use http_body_util::Full;
use hyper::{Request, Response};
use std::convert::Infallible;
use hyper::body::Bytes;

pub mod generator;

pub fn run(st: &str) {
    println!("Your code is: {}", st);
}


pub async fn handle_client(req: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let st = generator::utils::generate_client_id();
    run(&st[..]);
    Ok(Response::new(Full::new(Bytes::from("Here i am"))))
} 
