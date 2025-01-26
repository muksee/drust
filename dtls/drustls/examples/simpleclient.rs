use std::{
    io::{
        Read,
        Write,
    },
    net::TcpStream,
    sync::Arc,
};

use rustls::{
    self,
    crypto::CryptoProvider,
    pki_types::ServerName,
};

use rustls::crypto::aws_lc_rs as provider;

fn main() {
    // 1.创建客户端证书库
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
    );

    // 2.创建TLS客户端配置
    let config = rustls::ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: vec![
                provider::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256,
            ],
            kx_groups: vec![provider::kx_group::X25519],
            ..provider::default_provider()
        }
        .into(),
    )
    .with_protocol_versions(&[&rustls::version::TLS13])
    .unwrap()
    .with_root_certificates(root_store)
    .with_no_client_auth();

    // 3.创建TLS客户端端点
    let server_name = ServerName::try_from("www.rust-lang.org").unwrap();
    let mut conn =
        rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();

    // 4.创建TCP连接
    let mut sock = TcpStream::connect("www.rust-lang.org:443").unwrap();

    // 5.将TLS端点绑定到TCP连接，创建TLS网络客户端
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);

    // 6.发送HTTP报文
    let request = concat!(
        "GET / HTTP/1.1\r\n",
        "Host: www.rust-lang.org\r\n",
        "Connection: close\r\n",
        "Accept-Encoding: identity\r\n",
        "\r\n"
    );
    let _ = writeln!(&mut std::io::stdout(), "Send: {}", request);

    tls.write_all(request.as_bytes())
        .unwrap();

    // 7.打印TLS协商的加密套件
    let cipher_suite = tls
        .conn
        .negotiated_cipher_suite();
    let _ = writeln!(
        &mut std::io::stderr(),
        "Current cipher suite: {:?}",
        cipher_suite
    );

    // 8.HTTP读取
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext)
        .unwrap();
    let _ = writeln!(&mut std::io::stdout(), "Return: {} bytes", plaintext.len());
}
