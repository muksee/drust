//! HTTP2 服务端演示程序

use std::error::Error;

use bytes::Bytes;
use h2::{
    server::{
        self,
        SendResponse,
    },
    RecvStream,
};
use http::{
    Request,
    Response,
};
use log::*;
use tokio::net::{
    TcpListener,
    TcpStream,
};

type Result = std::result::Result<(), Box<dyn Error + Send + Sync>>;

const LISTEN_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() -> Result {
    // 日志组件
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init();

    // TCP启动监听
    let listener = TcpListener::bind(LISTEN_ADDR).await?;

    // TCP接受连接
    loop {
        if let Ok((socket, peer_addr)) = listener
            .accept()
            .await
        {
            info!("Receive tcp conn from {:?}", peer_addr);

            // 连接交给上层H2协议进行处理
            tokio::spawn(async {
                if let Err(e) = serve(socket).await {
                    error!("serve socket err: {:?}", e);
                }
            });
        }
    }
}

async fn serve(socket: TcpStream) -> Result {
    // H2建立连接
    let mut conn = server::handshake(socket).await?;
    info!("H2 connection bound");

    // H2接受下一个入栈请求
    // 每个请求对应一个Stream,多个Stream复用同一个连接,通过每个包对应的StreamID在连接中分拣各个流的数据.
    while let Some(result) = conn
        .accept()
        .await
    {
        // 每当分离出一个新的请求(新的Stream),就返回此Stream对应的请求和响应两个句柄,用于操作请求.
        let (request, respond) = result?;

        // 处理对应Stream的接收和响应
        // 每个Stream都启动单独的协程处理对应的业务,所有Stream的往返数据最终都是复用同一个连接进行传输.
        tokio::spawn(async {
            if let Err(e) = handle_request(request, respond).await {
                error!("error while handling request: {}", e);
            }
        });
    }

    // 当接收操作返回了,但是没有获取到新的请求,则表明收到了连接关闭的信号.
    log::info!("~~~~~~~~~~~~~~~~~~~ H2 connection CLOSE!!!! ~~~~~~~~~~~~~~~~~~~");
    Ok(())
}

async fn handle_request(
    mut request: Request<RecvStream>,
    mut respond: SendResponse<Bytes>,
) -> Result {
    info!("Got request: {:?}", request);

    // 从请求对象Request<T>中取出正文句柄(这里是RecvStream),用于后续读取真正的正文.
    let body = request.body_mut();

    // 从流中的下一个Data帧中读取请求正文
    while let Some(data) = body
        .data()
        .await
    {
        // 从流中读取一个帧
        let data = data?;
        info!("<<<< recv {:?}", data);

        // 通过流控,将帧所占额度从流中释放
        let _ = body
            .flow_control()
            .release_capacity(data.len());
    }

    // 创建响应正文
    let response = Response::new(());

    // 发送响应正文的Header帧,返回发送后续帧的句柄.标记为false表明流还未结束,后续还有帧.
    let mut send = respond.send_response(response, false)?;
    info!(">>>> send");

    // 发送响应正文的Data帧或Trailer帧,最后一个Data帧发送完毕后,标记Stream结束
    send.send_data(Bytes::from_static(b"Hello h2"), false)?;
    send.send_data(Bytes::from_static(b"H2"), true)?;

    Ok(())
}
