use std::error::Error;

use h2::client;
use http::{
    HeaderMap,
    Request,
};
use log::*;
use tokio::net::TcpStream;

const REMOTE_SERVER: &str = "127.0.0.1:3000";
const URI: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    info!("Try Connect to remote server: {:?}", REMOTE_SERVER);
    let tcp = TcpStream::connect("127.0.0.1:3000").await?;
    let peer_addr = tcp
        .peer_addr()
        .unwrap();
    let (mut client, h2) = client::handshake(tcp).await?;

    info!("Send Request: {:?}", peer_addr);
    let request = Request::builder()
        .uri(URI)
        .body(())
        .unwrap();

    let mut trailers = HeaderMap::new(); 
    trailers.insert(
        "zomg",
        "hello"
            .parse()
            .unwrap(),
    );

    let (response, mut stream) = client
        .send_request(request, false)
        .unwrap();
    stream
        .send_trailers(trailers)
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = h2.await {
            info!("Got ERR={:?}", e);
        }
    });

    let response = response.await?;
    info!("Got RESPONSE: {:?}", response);

    let mut body = response.into_body();

    while let Some(chunk) = body
        .data()
        .await
    {
        info!("Got CHUNCK = {:?}", chunk);
    }

    Ok(())
}
