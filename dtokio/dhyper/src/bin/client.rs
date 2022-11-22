use tokio::io::{stdout, AsyncWriteExt as _};

use hyper::{body::HttpBody, Body, Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;

extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    get().await?;
    get_body().await?;
    post().await?;
    multi_request().await?;
    get_with_https().await?;
    Ok(())
}

async fn get() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let p = "http://httpbin.org/ip";
    let uri = p.parse()?;

    let resp = client.get(uri).await?;

    println!("Response from {}-> {}", p, resp.status());

    Ok(())
}

async fn get_body() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let p = "http://httpbin.org/ip";
    let uri = p.parse()?;

    let mut resp = client.get(uri).await?;

    println!("Response from {}-> {}", p, resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        // tokio::io::stdout()
        stdout().write_all(&chunk?).await?;
    }

    Ok(())
}

async fn post() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 构建请求对象
    let p = "http://httpbin.org/post";
    let req = Request::builder()
        .method(Method::POST)
        .uri(p)
        .header("content-type", "appilcation/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;
    // 创建客户端,发起请求,打印结果
    let client = Client::new();
    let mut resp = client.request(req).await?;
    println!("Response from {}-> {}", p, resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
        stdout().write_u8(b'\n').await?;
    }

    Ok(())
}

async fn multi_request() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 客户端
    let client = Client::new();

    // 异步请求1
    let ip_fut = async {
        let resp = client
            .get(Uri::from_static("http://httpbin.org/ip"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    //异步请求2
    let headers_fut = async {
        let resp = client
            .get(Uri::from_static("http://httpbin.org/headers"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;
    println!("ip:{:?}\nheaders:{:?}", ip, headers);

    Ok(())
}

async fn get_with_https() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let p = "https://httpbin.org/ip";
    let uri = p.parse()?;

    let resp = client.get(uri).await?;

    println!("Response from {}-> {}", p, resp.status());

    Ok(())
}
