use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::convert::Infallible;
use hyper::body::Bytes;
use std::error::Error;

pub mod generator;

pub fn run(st: &str) {
    println!("Your code is: {}", st);
}


pub async fn handle_client(req: Request<impl hyper::body::Body + std::fmt::Debug>) -> Result<Response<Full<Bytes>>, Infallible> {

    if validate_req(&req) {
        route_req(&req);
    }

    let st = generator::utils::generate_client_id();
    run(&st[..]);
    Ok(Response::new(Full::new(Bytes::from("Here i am"))))
}

pub fn route_req(req: &Request<impl hyper::body::Body>) {

    match req.method() {
        &Method::GET => {
            if let Some(endpoint) = Endpoint::from_path(req.uri().path()) {
                match endpoint {
                    Endpoint::Token => println!("Token Endpoint"),
                    _ => println!("Unsuppported GET Endpoint"),
                }
            }
        }
        &Method::POST => {
            if let Some(endpoint) = Endpoint::from_path(req.uri().path()) {
                match endpoint {
                    Endpoint::Create => {
                        let c = match Client::new(req) {
                            Ok(client) => client,
                            Err(e) => 
                        }
                        println!("{}", Client::new(req));
                    }
                    Endpoint::Auth => println!("Creating auth"),
                    Endpoint::Token => println!("creating Token"),
                    Endpoint::Revoke => println!("Revoking Access"),
                }
            }
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
    Revoke,
    Create
}

impl Endpoint {
    fn as_str(&self) -> &'static str {
        match self {
            Endpoint::Token => "/oauth/v2/token",
            Endpoint::Auth => "/oauth/v2/auth",
            Endpoint::Revoke => "/oauth/v2/token/revoke",
            Endpoint::Create => "/user/create",

        }
    }

    fn from_path(path: &str) -> Option<Self> {
        match path {
            "/oauth/v2/token" => Some(Endpoint::Token),
            "/oauth/v2/auth" => Some(Endpoint::Auth),
            "/oauth/v2/token/revoke" => Some(Endpoint::Revoke),
            "/user/create" => Some(Endpoint::Create),
            _ => None,
        }
    }

    fn variants() -> &'static [Endpoint] {
        static VARIANTS: &[Endpoint] = &[
            Endpoint::Token,
            Endpoint::Auth,
            Endpoint::Revoke,
            Endpoint::Create,
        ];
        VARIANTS
    }
}


struct Client {

    name: String,
    homepage_url: String,
    redirect_url: String,
}


impl Client {
    fn new(&self, req: &Request<impl hyper::body::Body>) -> Result<Self, String> {
        Ok(Client {
            name: String::from("Michael"),
            homepage_url: String::from("www.homepage.com/"),
            redirect_url: String::from("www.redirect-url.com/"),
        })
    }
}


