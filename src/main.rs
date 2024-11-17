use hyper::{body::Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use oauth_server::handle_request;
use std::convert::Infallible;
use tokio::runtime::Runtime;

#[tokio::main]
fn main() {
    let addr = ([127, 0, 0, 1], 8081).into();

    let s = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(s);

    if let Err(e) = server.await {
        eprintln!("Server Error: {}", e);
    }
}
