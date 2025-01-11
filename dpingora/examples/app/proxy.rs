use std::sync::Arc;

use log::debug;
use pingora::{
    apps::ServerApp,
    connectors::TransportConnector,
    protocols::Stream,
    server::ShutdownWatch,
    upstreams::peer::BasicPeer,
};
use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt,
};

pub struct ProxyApp {
    client_connector: TransportConnector,
    proxy_to: BasicPeer,
}

enum DuplexEvent {
    DownstreamRead(usize),
    UpstreamRead(usize),
}

impl ProxyApp {
    pub fn new(proxy_to: BasicPeer) -> ProxyApp {
        ProxyApp {
            client_connector: TransportConnector::new(None),
            proxy_to,
        }
    }

    async fn duplex(
        &self,
        mut server_session: Stream,
        mut client_session: Stream,
    ) {
        let mut downstream_buf = [0; 1024];
        let mut upstream_buf = [0; 1024];

        loop {
            let downstream_read = server_session.read(&mut upstream_buf);
            let upstream_read = client_session.read(&mut downstream_buf);
            let event: DuplexEvent;

            tokio::select! {
                n = downstream_read => event = DuplexEvent::DownstreamRead(n.unwrap()),
                n = upstream_read => event = DuplexEvent::UpstreamRead(n.unwrap()),
            }

            match event {
                DuplexEvent::DownstreamRead(0) => {
                    debug!("downstream session closing");
                    return;
                }
                DuplexEvent::UpstreamRead(0) => {
                    debug!("upstream session closing");
                    return;
                }
                DuplexEvent::DownstreamRead(n) => {
                    client_session
                        .write_all(&upstream_buf[0..n])
                        .await
                        .unwrap();
                    client_session
                        .flush()
                        .await
                        .unwrap();
                }
                DuplexEvent::UpstreamRead(n) => {
                    server_session
                        .write_all(&downstream_buf[0..n])
                        .await
                        .unwrap();
                    server_session
                        .flush()
                        .await
                        .unwrap();
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl ServerApp for ProxyApp {
    async fn process_new(
        self: &Arc<Self>,
        io: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let client_session = self
            .client_connector
            .new_stream(&self.proxy_to)
            .await;
        match client_session {
            Ok(client_session) => {
                self.duplex(io, client_session)
                    .await;
                None
            }
            Err(e) => {
                debug!("failed to create client session: {}", e);
                None
            }
        }
    }
}
