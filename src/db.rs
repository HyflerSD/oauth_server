use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio;

use crate::client::Client;

#[derive(Debug)]
pub struct DB {
    pub pool: PooledConn
}


pub async fn setup() -> std::result::Result<DB, Box<dyn std::error::Error>> {

    dotenv().ok();

    let mut builder = OptsBuilder::new();
    builder =  builder
        .user(Some(dotenv::var("DB_USER").expect("No User Name")))
        .pass(Some(env::var("DB_PASS").expect("No Password")))
        .db_name(Some(env::var("DB_NAME").expect("DB_NAME not specified")));
    
    let pool = Pool::new(builder)?;
    let conn = pool.get_conn()?;

    Ok(DB{
        pool: conn
    })
}

impl DB {
    pub fn get_user(&self) -> Option<Client> {
        Some(Client::new())
    }

    pub fn save_client(&self, client: &Client) -> Result<(), String> {
        println!("DB: {:#?}", client);
        Ok(())
    }
}
