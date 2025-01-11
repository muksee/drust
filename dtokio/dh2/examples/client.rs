//! HTTP2 客户端演示程序

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

    // 创建TCP连接
    info!("Try Connect to remote server: {:?}", REMOTE_SERVER);
    let tcp = TcpStream::connect("127.0.0.1:3000").await?;
    let peer_addr = tcp
        .peer_addr()
        .unwrap();

    // 在连接上执行HTTP2握手
    let (mut client, h2) = client::handshake(tcp).await?;

    // 创建请求对象
    info!("Send Request: {:?}", peer_addr);
    let request = Request::builder()
        .uri(URI)
        .body(())
        .unwrap();

    // 创建拖尾对象
    let mut trailers = HeaderMap::new(); 
    trailers.insert(
        "zomg",
        "hello"
            .parse()
            .unwrap(),
    );

    // 发送请求的Header帧,和数据帧
    let (response, mut stream) = client
        .send_request(request, false)
        .unwrap();

    // 发送Trailer帧
    stream
        .send_trailers(trailers)
        .unwrap();

    // 监测HTTP2链路状态
    tokio::spawn(async move {
        if let Err(e) = h2.await {
            info!("Got ERR={:?}", e);
        }
    });

    // 读取响应
    let response = response.await?;
    info!("Got RESPONSE: {:?}", response);

    // 获取响应关联的Stream对象
    let mut body = response.into_body();

    // 接收数据帧
    while let Some(chunk) = body
        .data()
        .await
    {
        info!("Got CHUNCK = {:?}", chunk);
    }

    Ok(())
}
