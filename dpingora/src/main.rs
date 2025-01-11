use std::sync::Arc;

use pingora::{
    http::RequestHeader,
    lb::{
        health_check::TcpHealthCheck,
        selection::RoundRobin,
        LoadBalancer,
    },
    listeners::TlsSettings,
    proxy::{
        http_proxy_service,
        ProxyHttp,
        Session,
    },
    server::{
        configuration::Opt,
        Server,
    },
    services::{
        background::background_service,
        listening::Service,
    },
    upstreams::peer::HttpPeer,
};

use prometheus::{
    register_int_counter,
    IntCounter,
};

use async_trait::async_trait;
use pingora::Result;

fn main() {
    // 1.创建服务器实例
    let mut my_server = Server::new(Some(Opt::default())).unwrap();
    my_server.bootstrap();

    // 2.创建上游负载组
    let mut upstreams = LoadBalancer::try_from_iter([
        "127.0.0.1:3000",
        "127.0.0.1:3001",
        "127.0.0.1:3002",
    ])
    .unwrap();

    // 3.为上游负载组添加TCP健康检查
    let hc = TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    // 4.创建一个后台服务,用于对上游负载组进行健康检查
    let background = background_service("heath check", upstreams);
    // 获取后台服务对应的上游服务组,实际是原上游服务的Arc包装
    let upstreams = background.task();

    // 5.创建一个监听服务,负责接收下游请求,代理到上游
    let mut lb = http_proxy_service(
        &my_server.configuration,
        LB {
            lb: upstreams,
            req_metric: register_int_counter!(
                "reg_counter",
                "Number of requests"
            )
            .unwrap(),
        },
    );
    // 添加HTTP监听地址
    lb.add_tcp("0.0.0.0:6188");
    // 添加HTTPS监听地址
    let cert_path = format!("{}/keys/server.crt", env!("CARGO_MANIFEST_DIR"));
    let key_path = format!("{}/keys/key.pem", env!("CARGO_MANIFEST_DIR"));

    let mut tls_settings =
        TlsSettings::intermediate(&cert_path, &key_path).unwrap();
    tls_settings.enable_h2();
    lb.add_tls_with_settings("0.0.0.0:6189", None, tls_settings);

    // 6.将所有服务登记到服务器中,包括后台服务和代理服务
    my_server.add_service(background);
    my_server.add_service(lb);

    // 7.添加prometheues监控收集服务.必须注册键控key才能拉取到键控值
    // 收集地址: http://127.0.0.1:7188/metrics
    let mut prom = Service::prometheus_http_service();
    prom.add_tcp("0.0.0.0:7188");
    my_server.add_service(prom);

    // 8.启动服务器
    my_server.run_forever();
}

/// HTTP内部服务,必须实现ProxyHTTP
///
/// 主要功能:
/// - 选择后端地址 upstream_peer
/// - 处理请求过滤 upstream_request_filter
pub struct LB {
    lb: Arc<LoadBalancer<RoundRobin>>,
    req_metric: IntCounter,
}

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    /// 获取后端服务器
    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut (),
    ) -> Result<Box<HttpPeer>> {
        let upstream = self
            .lb
            .select(b"", 256)
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        let peer = Box::new(HttpPeer::new(upstream, false, "".to_string()));

        Ok(peer)
    }

    /// 客户端请求过滤
    async fn request_filter(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        println!("receive request");
        // 统计请求数指标加一
        self.req_metric
            .inc();
        Ok(false)
    }

    /// 向上游发起请求
    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        println!("request proxy");

        upstream_request
            .insert_header("Host", "one.one.one.one")
            .unwrap();

        Ok(())
    }
}
