use hyper::{
    body::to_bytes,
    client::{
        Client,
        HttpConnector,
    },
    Body,
    Request,
};
use std::ops::Deref;

extern crate tokio;

const URI: &str = "http://httpbin.org/ip";

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let connector = HttpConnector::new();
    let client = Client::builder().build::<_, Body>(connector);

    let request = Request::builder()
        .uri(URI)
        .body(Body::empty())
        .unwrap();

    let res = client
        .request(request)
        .await?;
    let status = res.status();
    let content = to_bytes(res)
        .await
        .unwrap();

    println!("status: {}", status);
    println!("content: {:?}", content);

    let s = "abc";
    let r = match s {
        "abc" => (),
        _ => (),
    };

    Ok(())
}
