use futures::future::join;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Request, Response};
use std::net::SocketAddr;
use tokio::net::TcpListener;

extern crate tokio;

static INDEX1: &[u8] = b"The 1st server!";
static INDEX2: &[u8] = b"The 2st server!";

async fn index1(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("server1 -> {:?}", req);
    Ok(Response::new(Body::from(INDEX1)))
}

async fn index2(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("server2 -> {:?}", req);
    Ok(Response::new(Body::from(INDEX2)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr1: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let addr2: SocketAddr = ([127, 0, 0, 1], 8001).into();

    let server1 = async move {
        let tcp_listener = TcpListener::bind(addr1).await.unwrap();
        println!("server 1 started to listening: {:?}", addr1);
        loop {
            let (stream, _) = tcp_listener.accept().await.unwrap();
            tokio::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(index1))
                    .await
                {
                    eprintln!("Failed to server connection for server1: {:?}", err);
                }
            });
        }
    };

    let server2 = async move {
        let tcp_listener = TcpListener::bind(addr2).await.unwrap();
        println!("server 2 started to listening: {:?}", addr2);
        loop {
            let (stream, _) = tcp_listener.accept().await.unwrap();
            tokio::spawn(async move {
                if let Err(err) = Http::new()
                    .serve_connection(stream, service_fn(index2))
                    .await
                {
                    eprintln!("Failed to server connection for server1: {:?}", err);
                }
            });
        }
    };

    let _ret = join(server1, server2).await;

    Ok(())
}
