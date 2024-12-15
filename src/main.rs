use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use oauth_server::handle_client;

pub mod db;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_pool = db::start().unwrap_or_else(|e| {
        println!("Error connecting to database: {:#?}", e);
        std::process::exit(1);
    });

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);
    let pool = db_pool.pool.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(move|req| handle_client(req, pool.clone())))
            .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
