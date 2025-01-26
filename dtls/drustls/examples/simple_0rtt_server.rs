//! 
//! 测试：
//! - 导出PSK：`openssl s_client -connect localhost:4443 -sess_out sess.pem`
//! - 使用PSK：`openssl s_client -connect localhost:4443 -sess_in sess.pem -early_data early.txt`
//! 
use std::{
    env,
    error::Error as StdError,
    io::{
        self,
        Read,
        Write,
    },
    net::TcpListener,
    sync::Arc,
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
    let mut config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;
    // ★ 必须设置，默认为0禁用早期设置。
    config.max_early_data_size = 1000;

    // 4.启动TCP监听,接受客户端请求
    let listener = TcpListener::bind(format!("0.0.0.0:{}", 4443)).unwrap();
    loop {
        let (mut stream, _) = listener.accept()?;
        println!("Accepting a connection");

        // 5.创建tls端点
        let mut conn = rustls::ServerConnection::new(Arc::new(config.clone()))?;

        let mut buf = Vec::new();
        let mut did_early_data = false;

        'handshake: while conn.is_handshaking() {
            while conn.wants_write() {
                if conn.write_tls(&mut stream)? == 0 {
                    stream.flush()?;
                    break 'handshake;
                }
            }

            while conn.wants_read() {
                match conn.read_tls(&mut stream) {
                    Ok(0) => {
                        return Err(
                            io::Error::from(io::ErrorKind::UnexpectedEof).into()
                        )
                    }
                    Ok(_) => break,
                    Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {}
                    Err(err) => return Err(err.into()),
                };
            }

            if let Err(e) = conn.process_new_packets() {
                let _ignored = conn.write_tls(&mut buf);
                stream.flush()?;

                return Err(io::Error::new(io::ErrorKind::InvalidData, e).into());
            }

            if let Some(mut early_data) = conn.early_data() {
                if !did_early_data {
                    println!("Receive early data from client");
                    did_early_data = true;
                }

                let bytes_read = early_data
                    .read_to_end(&mut buf)
                    .unwrap();

                if bytes_read != 0 {
                    println!("Early data from client: {:?}", buf);
                }
            }
        }

        if !did_early_data {
            println!("Did not receive early data from client");
        }

        println!("Handshare complete\n");

        conn.writer()
            .write_all(b"Hello from server")?;

        conn.send_close_notify();
        let _ = conn.complete_io(&mut stream);
    }
}
