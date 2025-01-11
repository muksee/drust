use pingora::{
    listeners::{
        TlsAccept,
        TlsSettings,
    },
    server::{
        configuration::Opt,
        Server,
    },
    services::{
        background::background_service,
        listening::Service as ListenerService,
        Service,
    },
    tls::{
        pkey::{
            PKey,
            Private,
        },
        ssl::{
            SslRef,
            SslVersion,
        },
        x509::X509,
    },
};

mod app;
mod service;

use service::{
    echo::{
        echo_service,
        echo_service_http,
    },
    proxy::{
        proxy_service,
        proxy_service_tls,
        ExampleBackgroundService,
    },
};

use structopt::StructOpt;

struct DynamicCert {
    cert: X509,
    key: PKey<Private>,
}

impl DynamicCert {
    fn new(cert: &str, key: &str) -> Box<Self> {
        let cert_bytes = std::fs::read(cert).unwrap();
        let cert = X509::from_pem(&cert_bytes).unwrap();

        let key_bytes = std::fs::read(key).unwrap();
        let key = PKey::private_key_from_pem(&key_bytes).unwrap();
        Box::new(DynamicCert { cert, key })
    }
}

#[async_trait::async_trait]
impl TlsAccept for DynamicCert {
    async fn certificate_callback(&self, ssl: &mut SslRef) {
        use pingora::tls::ext;
        ext::ssl_use_certificate(ssl, &self.cert).unwrap();
        ext::ssl_use_private_key(ssl, &self.key).unwrap();
    }
}

fn main() {
    // 服务器准备
    let opt = Some(Opt::from_args());
    let mut my_server = Server::new(opt).unwrap();
    my_server.bootstrap();

    // 证书文件
    let cert_path = format!("{}/keys/drustls-server-ca.pem", env!("CARGO_MANIFEST_DIR"));
    let key_path = format!("{}/keys/drustls-server-ca-key.pem", env!("CARGO_MANIFEST_DIR"));

    // TCP回声服务器
    // 1.TCP 2.TLS
    let mut echo_service = echo_service();
    echo_service.add_tcp("0.0.0.0:1080");
    echo_service
        .add_tls("0.0.0.0:1443", &cert_path, &key_path)
        .unwrap();

    // HTTP回声服务器
    // 1.HTTP 2.HTTPS 3.HTTPS with custom Settings
    let mut echo_service_http = echo_service_http();
    echo_service_http.add_tcp("0.0.0.0:2080");
    echo_service_http.add_uds("/tmp/echo.sock", None);
    let dynamic_cert = DynamicCert::new(&cert_path, &key_path);
    let mut tls_settings = TlsSettings::with_callbacks(dynamic_cert).unwrap();
    tls_settings
        .set_max_proto_version(Some(SslVersion::TLS1_2))
        .unwrap();
    tls_settings.enable_h2();
    echo_service_http.add_tls_with_settings("0.0.0.0:2433", None, tls_settings);

    // TCP代理
    // 1.TCP 2.TLS
    let proxy_service = proxy_service("0.0.0.0:3080", "54.165.134.201:80");
    let proxy_service_ssl = proxy_service_tls(
        "0.0.0.0:3443",
        "54.165.134.201:443",
        "httpbin.org ",
        &cert_path,
        &key_path,
    );

    // 后台服务
    let backgroud_service =
        background_service("example", ExampleBackgroundService::new());

    // Prometheus服务器
    let mut prometheus_service_http = ListenerService::prometheus_http_service();
    prometheus_service_http.add_tcp("0.0.0.0:4080");
    prometheus_service_http
        .add_tls("0.0.0.0:4443", &cert_path, &key_path)
        .unwrap();

    // 批量注册服务
    let services: Vec<Box<dyn Service>> = vec![
        Box::new(echo_service),
        Box::new(echo_service_http),
        Box::new(proxy_service),
        Box::new(proxy_service_ssl),
        Box::new(backgroud_service),
        Box::new(prometheus_service_http),
    ];

    my_server.add_services(services);
    my_server.run_forever();
}
