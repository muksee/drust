use std::{
    error::Error,
    net::ToSocketAddrs,
    sync::Arc,
};

use h2::client;
use http::{
    Method,
    Request,
};
use log::{
    error,
    info,
};
use tokio::net::TcpStream;
use tokio_rustls::{
    rustls::{
        pki_types::ServerName,
        RootCertStore,
    },
    TlsConnector,
};

const ALPH_H2: &str = "h2";
const SERVER_NAME: &str = "http2.akamai.com";
const SERVER_ADDR: &str = "http2.akamai.com:443";
const SERVER_URI: &str = "https://http2.akamai.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    let tls_client_config = Arc::new({
        let root_store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };

        let mut c = tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        c.alpn_protocols
            .push(
                ALPH_H2
                    .as_bytes()
                    .to_owned(),
            );
        c
    });

    let addr = SERVER_ADDR
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    let tcp = TcpStream::connect(&addr).await?;
    let dns_name = ServerName::try_from(SERVER_NAME).unwrap();
    let connector = TlsConnector::from(tls_client_config);
    let res = connector
        .connect(dns_name, tcp)
        .await;
    let tls = res.unwrap();
    {
        let (_, session) = tls.get_ref();
        let negotiated_protocol = session.alpn_protocol();
        assert_eq!(Some(ALPH_H2.as_bytes()), negotiated_protocol);
    }

    let (mut client, h2) = client::handshake(tls).await?;

    let request = Request::builder()
        .method(Method::POST)
        .uri(SERVER_URI)
        .body(())
        .unwrap();

    let (response, _) = client
        .send_request(request, true)
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = h2.await {
            error!("Got ERR = {:?}", e);
        }
    });

    let (_, mut body) = response
        .await?
        .into_parts();
    while let Some(chunk) = body
        .data()
        .await
    {
        info!("RX: {:?}", chunk?);
    }

    Ok(())
}
