use http_body_util::Full;
use url::{Url, ParseError};
use hyper::{Request, Response, Method};
use std::convert::Infallible;
use hyper::body::Bytes;
use std::error::Error;
use mysql::Pool;

pub mod client;
pub mod db;
pub mod generator;

use generator::utils;

pub fn run(st: &str) {
    println!("Your code is: {}", st);
}


pub async fn handle_client(req: Request<impl hyper::body::Body + std::fmt::Debug>, pool: Pool) -> Result<Response<Full<Bytes>>, Infallible> {
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            return Ok(Response::new(Full::new(Bytes::from("Here i am\n"))));
        }
    };

    let mut url = format!("http://www.w.com{}",req.uri());
    match Url::parse(&url[..]) {
        Ok(uri) => {
            println!("{:?}", uri.query_pairs().count());
        },
        Err(e) => {
            ()
        }
    }
    println!("{:?}", url);

    //if !validate_req(&req) {
    //    let response = format!("Invalid endpoint {}\n", req.uri().path());
    //    return Ok(Response::new(Full::new(Bytes::from(response))));
    //}
            Ok(Response::new(Full::new(Bytes::from(format!("Error: {}", "k".to_string())))))

    //match route_req(&req) {
    //    Ok(message) => {
    //        let val = format!("Code: {}", utils::generate_client_id());
    //        Ok(Response::new(Full::new(Bytes::from(val))))
    //    },
    //    Err(e) => {
    //        Ok(Response::new(Full::new(Bytes::from(format!("Error: {}", e)))))
    //    }
    //}

}

pub fn route_req(req: &Request<impl hyper::body::Body>) -> Result<String, String> {

    match req.method() {
        &Method::GET => {
            if let Some(endpoint) = Endpoint::from_path(req.uri().path()) {
                match endpoint {
                    Endpoint::Token => {
                        return Ok(utils::generate_client_id())
                    }
                    _ => {
                        println!("{:?}", endpoint);
                        return Err(String::from("error with route"))
                    }
                }
            }
        }
        &Method::POST => {
            if let Some(endpoint) = Endpoint::from_path(req.uri().path()) {

                match endpoint {
                    Endpoint::Create => {
                        println!("Create={:?}", endpoint);
                        return Ok(String::from("Client created"))
                    }
                    Endpoint::Auth => {
                        println!("Auth={:?}", endpoint);
                        return Ok(String::from("Auth Token"))
                    }
                    Endpoint::Revoke => {
                        println!("Revoke={:?}", endpoint);
                        return Ok(String::from("Revoked Access"))
                    }
                    _ => {
                        println!("Error EndPoint={:?}", endpoint);
                        return Err(String::from("Bad Post"))
                    }
                }
            }
        }
        _ => {
            println!("We dont supprt this method yet: {:?}", req.method());
            return Ok(String::from("Some error"))
        }
    }

    Err(String::from("Oh my goodness"))
}

pub fn validate_req(req: &Request<impl hyper::body::Body>) -> bool {
    if Endpoint::variants().iter().any(|endpoint| req.uri().path() == endpoint.as_str()) {
        return true
    }
    false
}


#[derive(Debug)]
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


