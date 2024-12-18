use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::error::Error;
use mysql::PooledConn;
use crate::generator::utils;

#[derive(Debug, PartialEq, Eq)]
pub struct Client {

    name: String,
    client_id: String,
    client_secret: String,

}

impl Client {
    pub fn new() -> Self {
        Client {
            name: String::from("Michael"),
            client_id: String::from("www.homepage.com/"),
            client_secret: String::from("www.redirect-url.com/"),
        }
    }

    pub fn from(req: &Request<impl hyper::body::Body>) -> Result<Self, String> {
        Ok(Client {
            name: String::from("Michael"),
            client_id: String::from("www.homepage.com/"),
            client_secret: String::from("www.redirect-url.com/"),
        })
    }
}

pub struct ClientRepository<'a> {
    db_pool: &'a PooledConn,
}

impl<'a> ClientRepository<'a> {
    pub fn new(db_pool: &'a PooledConn) -> Self {
        Self { db_pool }
    }

    pub fn create(&self, req: &Client) -> Result<(), String> {
        Ok(())
        //Ok(Client::new());
    }

    pub fn delete(&self, client: &Client) -> Result<(), String> {
        Ok(())
    }

    pub fn exists(&self, email: String) -> bool {
        false
    }

    pub fn save(&self) -> Result<(), String> {
        //self.save_client(client);
        Ok(())
    }

    pub fn validate_client(&self, client_id: &str, client_secret: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn revoke(&self, client: &Client) -> Result<(), String> {
        Ok(())
    }

    pub fn get_user(&self) -> Option<Client> {
        Some(Client::new())
    }

}

