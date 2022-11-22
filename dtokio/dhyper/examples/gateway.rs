extern crate tokio;
use std::future::Future;
use std::net::SocketAddr;

use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let in_addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    let tcp_listener = TcpListener::bind(in_addr).await?;

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let service = service_fn(proxy);
        tokio::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service).await {
                eprintln!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

fn proxy(mut req: Request<Body>) -> impl Future<Output = Result<Response<Body>, hyper::Error>> {
    let uri_string = format! {
        "http://{}{}",
        "httpbin.org",
        req.uri().path_and_query().map(|x|x.as_str()).unwrap_or("/")
    };
    let uri = uri_string.parse().unwrap();
    *req.uri_mut() = uri;

    let host = req.uri().host().expect("uri has no host");
    let port = req.uri().port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    async move {
        let stream = TcpStream::connect(addr).await.unwrap();
        let (mut sender, conn) = hyper::client::conn::handshake(stream).await?;
        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connect failed:{:?}", err);
            }
        });

        sender.send_request(req).await
    }
}
