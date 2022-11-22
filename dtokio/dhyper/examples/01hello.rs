use std::{convert::Infallible, net::SocketAddr};

use hyper::service::service_fn;
use hyper::{server::conn::Http, Body, Request, Response};
use tokio::net::TcpListener;

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = tcp_listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                eprintln!("Faile to server connecton: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello Hyper")))
}
