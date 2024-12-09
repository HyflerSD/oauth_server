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
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    let pool = match db::setup().await {
        Ok(p) => p,
        Err(e) => {
            println!("{:?}", e);
            return Ok(Response::new(Full::new(Bytes::from("500 Server Error\n".to_string()))))
        }
    };

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_client))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
