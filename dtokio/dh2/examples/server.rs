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
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    let listener = TcpListener::bind(LISTEN_ADDR).await?;

    loop {
        if let Ok((socket, peer_addr)) = listener
            .accept()
            .await
        {
            info!("Receive tcp conn from {:?}", peer_addr);
            tokio::spawn(async {
                if let Err(e) = serve(socket).await {
                    error!("serve socket err: {:?}", e);
                }
            });
        }
    }
}

async fn serve(socket: TcpStream) -> Result {
    let mut conn = server::handshake(socket).await?;
    info!("H2 connection bound");

    while let Some(result) = conn
        .accept()
        .await
    {
        let (request, respond) = result?;
        tokio::spawn(async {
            if let Err(e) = handle_request(request, respond).await {
                error!("error while handling request: {}", e);
            }
        });
    }

    log::info!("~~~~~~~~~~~~~~~~~~~ H2 connection CLOSE!!!! ~~~~~~~~~~~~~~~~~~~");
    Ok(())
}

async fn handle_request(
    mut request: Request<RecvStream>,
    mut respond: SendResponse<Bytes>,
) -> Result {
    info!("Got request: {:?}", request);

    let body = request.body_mut();
    while let Some(data) = body
        .data()
        .await
    {
        let data = data?;
        info!("<<<< recv {:?}", data);

        let _ = body
            .flow_control()
            .release_capacity(data.len());
    }

    let response = Response::new(());

    let mut send = respond.send_response(response, false)?;
    info!(">>>> send");

    send.send_data(Bytes::from_static(b"Hello h2"), false)?;
    send.send_data(Bytes::from_static(b"H2"), true)?;

    Ok(())
}
