use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env::{self};
use tokio;

pub fn hello() {
    println!("hellow world");
}

pub async fn setup() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut builder = OptsBuilder::new();
    builder =  builder
        .user(Some(env::var("DB_USER").expect("No User Name")))
        .pass(Some(env::var("DB_PASS").expect("No Password")))
        .db_name(Some(env::var("DB_NAME").expect("DB_NAME not specified")));
    
    let pool = Pool::new(builder)?;
    match pool.get_conn() {
        Ok(_) => println!("Connected!"),
        Err(e) => println!("{:?}", e)
    }
    Ok(())
}
