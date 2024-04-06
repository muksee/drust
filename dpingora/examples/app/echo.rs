use std::{
    sync::Arc,
    time::Duration,
};

use bytes::Bytes;
use http::{
    Response,
    StatusCode,
};
use log::debug;
use once_cell::sync::Lazy;
use pingora::{
    apps::{
        http_app::ServeHttp,
        ServerApp,
    },
    protocols::{
        http::ServerSession,
        Stream,
    },
    server::ShutdownWatch,
};

use prometheus::{
    register_int_counter,
    IntCounter,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    time::timeout,
};

#[derive(Clone)]
pub(crate) struct EchoApp;

impl EchoApp {
    pub fn new() -> Arc<EchoApp> {
        Arc::new(EchoApp)
    }
}

#[async_trait::async_trait]
impl ServerApp for EchoApp {
    async fn process_new(
        self: &Arc<Self>,
        mut io: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let mut buf = [0; 1024];
        loop {
            let n = io
                .read(&mut buf)
                .await
                .unwrap();
            if n == 0 {
                debug!("session is closing");
                return None;
            }

            io.write_all(&buf[0..n])
                .await
                .unwrap();
            io.flush()
                .await
                .unwrap();
        }
    }
}

static REQ_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!("req_counter", "Number of requests").unwrap()
});

pub(crate) struct HttpEchoApp;

impl HttpEchoApp {
    pub fn new() -> Arc<HttpEchoApp> {
        Arc::new(HttpEchoApp)
    }
}

#[async_trait::async_trait]
impl ServeHttp for HttpEchoApp {
    async fn response(
        &self,
        http_stream: &mut ServerSession,
    ) -> Response<Vec<u8>> {
        REQ_COUNTER.inc();

        let read_timeout = 2000;

        let body = timeout(
            Duration::from_millis(read_timeout),
            http_stream.read_request_body(),
        )
        .await;

        let body = match body {
            Ok(res) => match res.unwrap() {
                Some(bytes) => bytes,
                None => Bytes::from("empty body!"),
            },
            Err(_) => {
                panic!("Timed out after {:?}ms", read_timeout)
            }
        };

        Response::builder()
            .status(StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "text/html")
            .header(http::header::CONTENT_LENGTH, body.len())
            .body(body.to_vec())
            .unwrap()
    }
}
