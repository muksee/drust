//! # 编写一个HTTP客户端
//! 
//! 步骤
//! * 创建HTTP连接器 Connector
//! * 创建HTTP对端 HttpPeer
//! * 开始连接，获取会话 HttpSession
//! * 发送HTTP头部，发送HTTP正文
//! * 接收HTTP头部，接收HTTP正文
//! * 从HttpSession中取出已接收到的响应头部、正文
//! 
use pingora::{
    connectors::http::Connector,
    http::RequestHeader,
    upstreams::peer::HttpPeer,
};

const PEER_HOST: &str = "httpbin.org";
const PEER_HOST_PORT: &str = concat!("httpbin.org", ":", "443");

#[tokio::main]
async fn main() {
    // 1. 创建HTTP连接器 Connector
    let connector = Connector::new(None);

    // 2. 创建对端 HttpPeer
    let mut peer = HttpPeer::new(PEER_HOST_PORT, true, PEER_HOST.into());
    peer.options
        .set_http_version(2, 1);

    // 3. 连接对端，获取会话 HttpSession
    let (mut session, _reused) = connector
        .get_http_session(&peer)
        .await
        .unwrap();

    // 4. 发送HTTP请求头部、正文
    let mut new_request = RequestHeader::build("GET", b"/", None).unwrap();
    new_request
        .insert_header("Host", PEER_HOST)
        .unwrap();

    session
        .write_request_header(Box::new(new_request))
        .await
        .unwrap();
    session
        .finish_request_body()
        .await
        .unwrap();

    // 5. 接收HTTP响应头部、正文
    session
        .read_response_header()
        .await
        .unwrap();
    let body = session
        .read_response_body()
        .await
        .unwrap();


    // 8. 取出响应头部
    let header = &session
        .response_header()
        .unwrap();

    println!("Response Header: {:#?}", header);
    println!("Response Body: {:#?}", body);
}
