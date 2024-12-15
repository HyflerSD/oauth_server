use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio;
use log::{info, error, debug};

#[derive(Debug, Clone)]
pub struct DbPool {
    pub pool: Pool
}

pub fn start() -> std::result::Result<DbPool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut builder = OptsBuilder::new();
    builder =  builder
        .user(Some(dotenv::var("DB_USER").expect("No User Name")))
        .pass(Some(env::var("DB_PASS").expect("No Password")))
        .db_name(Some(env::var("DB_NAME").expect("DB_NAME not specified")));
    
    let pool = Pool::new(builder)?;
    let mut conn = pool.get_conn()?;
    match DbPool::init(&mut conn) {
        Ok(p) => {
            Ok(DbPool{
                pool: pool
            })
        },
        Err(e) => {
            println!("Error connecting to database:\n {:#?}\n", e);
            Err(Box::new(e))
        }
    }
}

impl DbPool {

    pub fn init(conn: &mut PooledConn) -> Result<(), Error> {
        match Self::create_clients_table(conn) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Init Erorr:\n {:#?}\n", e);
                Err(e)
            }
        }
    }

    pub fn create_clients_table(conn: &mut PooledConn) -> Result<(), Error> {

        match Self::table_exists(conn) {
            Err(e) => Err(e),
            Ok(v) => {
                if v > Some(0) {
                    return Ok(());
                }
                    conn.query_drop(
                        "CREATE TABLE IF NOT EXISTS clients (
                        id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY, 
                        client_id varchar(54) NOT NULL,
                        allowed_scopes VARCHAR(28) NULL,
                        client_name VARCHAR(128) NOT NULL,
                        client_secret VARCHAR(54) NOT NULL,
                        grant_types VARCHAR(19) NULL
                        )")?;
                Ok(())
            },
        }
    }
    pub fn table_exists(conn: &mut PooledConn) -> Result<Option<u64>, Error> {
        let table = "clients";
        let stmt = format!("SELECT COUNT(*) FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME='{}'", table);
        let query_res: Result<Option::<u64>> = conn.query_first(stmt);
        match query_res {
            Ok(res) => {
                //println!("go{:?}", res);
                Ok(res)
            },
            Err(e) =>{
                //println!("b{:?}", e);
                Err(e)
            }
        }
    }
}


