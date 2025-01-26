//!
//! curl -vvv -k --http0.9 -X POST -d "hello" https://localhost:4443
//! openssl s_client -connect localhost:4443
use std::{
    env,
    error::Error as StdError,
    io::{
        Read,
        Write,
    },
    net::TcpListener,
    sync::Arc,
    thread,
    time::Duration,
};

use rustls::pki_types::{
    pem::PemObject,
    CertificateDer,
    PrivateKeyDer,
};

fn main() -> Result<(), Box<dyn StdError>> {
    // 1.准备服务器证书和私钥
    let cert_file =
        format!("{}/keys/drustls-server-ca.pem", env!("CARGO_MANIFEST_DIR"));
    let private_key_file =
        format!("{}/keys/drustls-server-ca-key.pem", env!("CARGO_MANIFEST_DIR"));

    // 2.加载服务器证书和私钥
    let certs = CertificateDer::pem_file_iter(cert_file)
        .unwrap()
        .map(|cert| cert.unwrap())
        .collect();
    let private_key = PrivateKeyDer::from_pem_file(private_key_file).unwrap();

    // 3.创建服务器tls配置
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;

    // 4.启动TCP监听,接受客户端请求
    let listener = TcpListener::bind(format!("0.0.0.0:{}", 4443)).unwrap();
    let (mut stream, _) = listener.accept()?;

    // 5.创建tls端点，并与连接绑定
    let mut conn = rustls::ServerConnection::new(Arc::new(config))?;
    conn.complete_io(&mut stream)?;

    conn.writer()
        .write_all(b"Hello from the server")?;

    conn.complete_io(&mut stream)?;

    let mut buf = [0; 64];
    loop {
        thread::sleep(Duration::from_secs(2));
        if conn.wants_read() {
            match conn
                .reader()
                .read(&mut buf)
            {
                Ok(len) => {
                    println!("Received message from client: {:?}", &buf[..len]);
                    break;
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // 等待连接准备好读取数据
                    conn.complete_io(&mut stream)?;
                }
                Err(e) => return Err(e.into()),
            }
        } else {
            conn.complete_io(&mut stream)?;
        }
    }

    Ok(())
}
