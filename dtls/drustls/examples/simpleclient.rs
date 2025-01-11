use std::{
    fs::File,
    io::{
        stdout,
        BufReader,
        Read,
        Write,
    },
    net::TcpStream,
    sync::Arc,
};

use rustls::{
    self,
    ClientConfig,
    RootCertStore,
};
use webpki_roots::TLS_SERVER_ROOTS;

fn main() {
    // 创建根证书容器,内含Mozilla信任的所有根证书
    let mut root_store = RootCertStore {
        roots: TLS_SERVER_ROOTS.into(),
    };

    // 追加自签名根证书
    let self_singed_cert_file =
        format!("{}/keys/drust-ca.pem", env!("CARGO_MANIFEST_DIR"));
    let mut buf_reader =
        BufReader::new(File::open(self_singed_cert_file).unwrap());
    let cert = rustls_pemfile::certs(&mut buf_reader).filter_map(Result::ok);
    root_store.add_parsable_certificates(cert);

    // 创建所有连接使用的TLS配置
    // 1.加载了根证书
    // 2.关闭客户端验证
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // 创建存储最终的对称加密秘钥的文件,从SSLKEYLOGFILE获取文件名
    // config.key_log = Arc::new(rustls::KeyLogFile::new());

    // 服务器名,用于SNI
    let server_name = "www.drustls.com"
        .try_into()
        .unwrap();

    // 创建一个TLS客户端连接对象: 配置 + 服务器名
    let mut conn =
        rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    // 创建一个传输层TCP连接对象
    let mut sock = TcpStream::connect("127.0.0.1:4443").unwrap();
    // 将TLS连接与传输层封装到一起，使用户可以像常规的流一样使用rustls连接
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);

    // 发送数据
    let result = tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: www.rust-lang.org\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    );

    println!("{:?}", result);

    // 协商加密套件
    let cipher_suite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "current cipher suites: {:?}",
        cipher_suite.suite()
    )
    .unwrap();

    // 接收数据
    let mut planin_text = Vec::new();
    let _ = tls.read_to_end(&mut planin_text);
    stdout()
        .write_all(&planin_text)
        .unwrap();
}
