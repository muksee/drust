use std::net::SocketAddr;

use hyper::{server::conn::Http, service::service_fn, Body, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

extern crate tokio;

static INDEX: &str = "examples/send_file.html";
static NOTFOUND: &[u8] = b"Not Found";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        tokio::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(response_example))
                .await
            {
                eprintln!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn response_example(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/no_file.html") => simple_file_send("This file shouldn't exist!").await,
        _ => Ok(not_found()),
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(file: &str) -> Result<Response<Body>, hyper::Error> {
    if let Ok(contents) = tokio::fs::read(file).await {
        return Ok(Response::new(contents.into()));
    }

    Ok(not_found())
}
