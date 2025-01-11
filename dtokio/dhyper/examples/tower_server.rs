use std::net::SocketAddr;
use std::time::Duration;

use futures::future;

use hyper::{server::conn::Http, service::Service, Body, Request, Response};
use tokio::{net::TcpListener, select, time::interval};

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let tcp_listener = TcpListener::bind(addr).await?;

    println!("listener on {:?}, will shutdown after 10s", addr);

    let (tx, mut rx) = tokio::sync::watch::channel("Hello");
    let mut interval = interval(Duration::from_secs(1));

    tokio::spawn(async move {
        for i in (0..=10).rev() {
            interval.tick().await;
            println!("{}", i);
        }
        let _ = tx.send("world");
    });

    loop {
        select! {
            Ok((stream, _)) = tcp_listener.accept() => {
                tokio::spawn(async move {
                    if let Err(err) = Http::new().serve_connection(stream, Svc).await {
                        eprintln!("Failed to server connection: {:?}", err);
                    }
                })},
            _ = rx.changed() => {
                break;
            }
        };
    }

    Ok(())
}

struct Svc;

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let rsp = Response::builder();

        if req.uri().path() != "/" {
            let body = Body::from(Vec::new());
            let rsp = rsp.status(hyper::StatusCode::NOT_FOUND).body(body).unwrap();

            return future::ok(rsp);
        }
        let body = Body::from("Hello,tower server!");
        let rsp = rsp.status(hyper::StatusCode::OK).body(body).unwrap();

        future::ok(rsp)
    }
}
