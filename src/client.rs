use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::error::Error;

#[derive(Debug)]
pub struct Client {

    name: String,
    homepage_url: String,
    redirect_url: String,
}


impl Client {
    pub fn new(req: &Request<impl hyper::body::Body>) -> Result<Self, String> {
        //Need to check if client exists already when db is here dude
        Ok(Client {
            name: String::from("Michael"),
            homepage_url: String::from("www.homepage.com/"),
            redirect_url: String::from("www.redirect-url.com/"),
        })
    }

    fn delete(&self) -> Result<(), String> {
        Ok(())
    }

    fn validate_client(client_id: &str, client_secret: &str) -> Result<(), String> {
        Ok(())
    }

    fn revoke(&self) -> Result<(), String> {
        Ok(())
    }
}

