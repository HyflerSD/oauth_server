use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::convert::Infallible;
use hyper::body::Bytes;

pub mod generator;

pub fn run(st: &str) {
    println!("Your code is: {}", st);
}


pub async fn handle_client(req: Request<impl hyper::body::Body + std::fmt::Debug>) -> Result<Response<Full<Bytes>>, Infallible> {
    match req.method() {
        &Method::GET => {
            //println!("Body: {:?}", req.body());
            let (parts, body) = req.into_parts();
            println!("{:?}", parts.uri);
        }
        &Method::POST => {
            println!("This is a post")
        }
        &Method::PATCH => {
            println!("This is a patch")
        }
        &Method::DELETE => {
            println!("This is a delete")
        }
        &Method::PUT => {
            println!("This is a put")
        }
        _ => println!("We dont supprt this method yet: {:?}", req.method())
    }

    let st = generator::utils::generate_client_id();
    run(&st[..]);
    Ok(Response::new(Full::new(Bytes::from("Here i am"))))
} 
