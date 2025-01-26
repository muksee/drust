use std::{
    io::{
        BufRead,
        BufReader,
        Write,
    },
    net::TcpStream,
    sync::Arc,
};

use rustls::{
    pki_types::ServerName,
    RootCertStore,
};
fn start_connection(
    config: &Arc<rustls::ClientConfig>,
    domain_name: &str,
    port: u16,
) {
    let server_name = ServerName::try_from(domain_name)
        .expect("invalid DNS name")
        .to_owned();
    let mut conn =
        rustls::ClientConnection::new(Arc::clone(config), server_name).unwrap();

    let mut sock =
        TcpStream::connect(format!("{}:{}", domain_name, port)).unwrap();

    // 设置套接字属性：立即发送。(对应的是攒够一批再发送)
    sock.set_nodelay(true)
        .unwrap();

    let request = format!(
        "GET / HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\
         Accept-Encoding: identity\r\n\
         \r\n",
        domain_name
    );

    // 如果客户端配置支持早期数据，则通过早期数据发送请求
    if let Some(mut early_data) = conn.early_data() {
        early_data
            .write_all(request.as_bytes())
            .unwrap();
        println!("* 0-RTT request sent");
    }

    // 捆绑tls客户端断端点和TCP链路
    let mut stream = rustls::Stream::new(&mut conn, &mut sock);

    // 结束握手
    stream
        .flush()
        .unwrap();

    // 如果服务器没有接受早期数据，或客户端配置不支持早期数据则，正常发送数据
    if !stream
        .conn
        .is_early_data_accepted()
    {
        stream
            .write_all(request.as_bytes())
            .unwrap();
        println!("* Normal request sent");
    } else {
        println!("* 0-RTT data accepted");
    }

    let mut first_response_line = String::new();
    BufReader::new(stream)
        .read_line(&mut first_response_line)
        .unwrap();
    println!("* Server response: {:?}", first_response_line);
}

fn main() {
    let domain_name = "jbp.io";
    let port = 443u16;

    let mut root_store = RootCertStore::empty();
    // root_store.add_parsable_certificates(
    //     CertificateDer::pem_file_iter(format!(
    //         "{}/keys/drust-ca.pem",
    //         env!("CARGO_MANIFEST_DIR")
    //     ))
    //     .expect("Cant open CA file")
    //     .map(|result| result.unwrap()),
    // );
    root_store.extend(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
    );

    let mut config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    config.key_log = Arc::new(rustls::KeyLogFile::new());

    config.enable_early_data = true;

    let config = Arc::new(config);

    println!("* Sending first request:");
    start_connection(&config, domain_name, port);
    println!("* Sending second request:");
    start_connection(&config, domain_name, port);
}
