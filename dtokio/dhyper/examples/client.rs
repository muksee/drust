use hyper::{body::HttpBody, Body, Request};
use std::env;
use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream,
};

extern crate tokio;

use hyper::client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };
    let uri: hyper::Uri = url.parse().unwrap();
    if uri.scheme_str() != Some("http") {
        println!("only support http");
        return Ok(());
    }

    fetch_uri(uri).await
}

async fn fetch_uri(uri: hyper::Uri) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);

    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).await?;

    let (mut sender, conn) = hyper::client::conn::handshake(stream).await?;

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {:?}", err);
        }
    });

    // ****注意Host头必须加,很多Web服务器没有Host头直接报400****
    let req = Request::builder()
        .uri(uri.clone())
        .header("Host", host)
        .body(Body::empty())
        .unwrap();
    let mut res = sender.send_request(req).await?;

    while let Some(next) = res.data().await {
        let chunk = next?;
        println!("{}", chunk.len());
        let _ = io::stdout().write(&chunk).await;
    }

    Ok(())
}
