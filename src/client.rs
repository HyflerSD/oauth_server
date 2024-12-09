use http_body_util::Full;
use hyper::{Request, Response, Method};
use std::error::Error;
use mysql::PooledConn;

use crate::db::DB;

#[derive(Debug, PartialEq, Eq)]
pub struct Client {

    name: String,
    client_id: String,
    client_secret: String,

}

pub struct ClientRepository<'a> {
    db_conn: &'a DB,
}

impl<'a> ClientRepository<'a> {
    pub fn new(db_conn: &'a DB) -> Self {
        Self { db_conn }
    }

    pub fn create(&self, req: &Client) -> Result<(), String> {
        Ok(())
    }

    pub fn create_and_save(client: &Client ) -> Result<(), String> {
        //Ok( Self { db_conn })
        Ok(())
    }

    pub fn delete(&self, client: &Client) -> Result<(), String> {
        Ok(())
    }

    pub fn save(&self, client: &Client) -> Result<(), String> {
        self.db_conn.save_client(client);
        Ok(())
    }

    pub fn validate_client(&self, client_id: &str, client_secret: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn revoke(&self, client: &Client) -> Result<(), String> {
        Ok(())
    }
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

