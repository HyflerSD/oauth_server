use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::convert::Infallible;
use hyper::body::Bytes;

pub mod generator;



pub fn run(st: &str) {
    println!("Your code is: {}", st);
}


pub async fn handle_client(req: Request<impl hyper::body::Body + std::fmt::Debug>) -> Result<Response<Full<Bytes>>, Infallible> {

    println!("{}", validate_req(&req));

    match req.method() {
        &Method::GET => {
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

pub fn validate_req(req: &Request<impl hyper::body::Body>) -> bool {
    if Endpoint::variants().iter().any(|endpoint| req.uri().path() == endpoint.as_str()) {
        return true;
    }
    false
}


enum Endpoint {
    Token,
    Auth,
    Revoke
}

impl Endpoint {
    fn as_str(&self) -> &'static str {
        match self {
            Endpoint::Token => "/oauth/v2/token",
            Endpoint::Auth => "/oauth/v2/auth",
            Endpoint::Revoke => "/oauth/v2/token/revoke",

        }
    }

    fn variants() -> &'static [Endpoint] {
        static VARIANTS: &[Endpoint] = &[
            Endpoint::Token,
            Endpoint::Auth,
            Endpoint::Revoke
        ];
        VARIANTS
    }
}

