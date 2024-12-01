use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::error::Error;
use mysql::PooledConn;

use crate::db::DB;

#[derive(Debug, PartialEq, Eq)]
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

    pub fn _new() -> Client {
        //Need to check if client exists already when db is here dude
        Client {
            name: String::from("_mmmm"),
            homepage_url: String::from("_www.homepage.com/"),
            redirect_url: String::from("_www.redirect-url.com/"),
        }
    }

    fn delete(&self) -> Result<(), String> {
        Ok(())
    }

    fn save(&self, dbconn: DB) -> Result<(), String> {
        dbconn.save_client(&self)
    }

    fn validate_client(client_id: &str, client_secret: &str) -> Result<(), String> {
        Ok(())
    }

    fn revoke(&self) -> Result<(), String> {
        Ok(())
    }
}

