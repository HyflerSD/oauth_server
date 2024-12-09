use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio;
use log::{info, error, debug};

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
    let mut conn = pool.get_conn()?;
    DB::init(&mut conn);
    Ok(DB{
        pool: conn
    })
}

impl DB {

    pub fn init(conn: &mut PooledConn) -> Result<(), Error> {
        match Self::create_clients_table(conn) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("{:#?}", &e);
                Err(e)
            }
        }
    }

    pub fn create_clients_table(conn: &mut PooledConn) -> Result<(), Error> {

        match Self::table_exists(conn) {
            Some(v) => {
                if !v {
                println!("not found");
                    conn.query_drop(
                        "CREATE TABLE IF NOT EXISTS clients (
                        id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY, 
                        client_id varchar(54) NOT NULL,
                        allowed_scopes VARCHAR(28) NULL,
                        client_name VARCHAR(128) NOT NULL,
                        client_secret VARCHAR(54) NOT NULL,
                        grant_types VARCHAR(19) NULL
                        )")?;
                    conn.query_drop(
                        "CREATE INDEX idx_client_id
                         ON cients (client_id)
                         )")?;
                    }
            },
            _ => ()
        }
        Ok(())
    }
    pub fn table_exists(conn: &mut PooledConn) -> Option<bool> {
        let table = "clients";
        let stmt = format!("SELECT COUNTS(*) FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME='{}'", table);
        let query_res: Result<Option::<u64>> = conn.query_first(stmt);
        match query_res {
            Ok(res) => {
                Some(true)
            },
            Err(e) =>{
                eprintln!("{:#?}", e);
                Some(false)
            }
        }
    }
    pub fn get_user(&self) -> Option<Client> {
        Some(Client::new())
    }

    pub fn save_client(&self, client: &Client) -> Result<(), String> {
        println!("DB: {:#?}", client);

        Ok(())
    }
}
