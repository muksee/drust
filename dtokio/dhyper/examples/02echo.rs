use std::net::SocketAddr;

use hyper::{server::conn::Http, service::service_fn, Body, Method, Request, Response};
use tokio::net::TcpListener;

extern crate tokio;

static HELP: &str =
    "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = tcp_listener.accept().await?;

        tokio::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                eprintln!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(HELP))),
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let reversed = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
