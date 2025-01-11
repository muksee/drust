use std::{
    sync::Arc,
    time::Duration,
};

use pingora::{
    listeners::Listeners,
    server::ShutdownWatch,
    services::{
        background::BackgroundService,
        listening::Service,
    },
    upstreams::peer::BasicPeer,
};
use tokio::time::interval;

use crate::app::proxy::ProxyApp;

pub fn proxy_service(addr: &str, proxy_addr: &str) -> Service<ProxyApp> {
    let proxy_to = BasicPeer::new(proxy_addr);

    Service::with_listeners(
        "Proxy Service".to_string(),
        Listeners::tcp(addr),
        Arc::new(ProxyApp::new(proxy_to)),
    )
}

pub fn proxy_service_tls(
    addr: &str,
    proxy_addr: &str,
    proxy_sni: &str,
    cert_path: &str,
    key_path: &str,
) -> Service<ProxyApp> {
    let mut proxy_to = BasicPeer::new(proxy_addr);
    // set SNI to enable TLS
    proxy_to.sni = proxy_sni.into();
    Service::with_listeners(
        "Proxy Service TLS".to_string(),
        Listeners::tls(addr, cert_path, key_path).unwrap(),
        Arc::new(ProxyApp::new(proxy_to)),
    )
}

pub struct ExampleBackgroundService;

impl ExampleBackgroundService {
    pub fn new() -> ExampleBackgroundService {
        ExampleBackgroundService
    }
}

#[async_trait::async_trait]
impl BackgroundService for ExampleBackgroundService {
    async fn start(&self, mut shutdown: ShutdownWatch) {
        let mut period = interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    break;
                }
                _ = period.tick() => {
                    println!("doing background work");
                }
            }
        }
    }
}
