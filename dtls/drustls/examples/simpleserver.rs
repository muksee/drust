use std::{
    env,
    error::Error as StdError,
    fs::File,
    io::{
        BufReader,
        Read,
        Write,
    },
    net::TcpListener,
    sync::Arc,
};



fn main() -> Result<(), Box<dyn StdError>> {
    let cert_file =
        format!("{}/keys/drustls-server-ca.pem", env!("CARGO_MANIFEST_DIR"));
    let private_key_file = format!(
        "{}/keys/drustls-server-ca-key.pem",
        env!("CARGO_MANIFEST_DIR")
    );

    let certs =
        rustls_pemfile::certs(&mut BufReader::new(&mut File::open(cert_file)?))
            .collect::<Result<Vec<_>, _>>()?;
    let private_key = rustls_pemfile::private_key(&mut BufReader::new(
        &mut File::open(private_key_file)?,
    ))?
    .unwrap();
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", 4443)).unwrap();
    let (mut stream, _) = listener.accept()?;

    let mut conn = rustls::ServerConnection::new(Arc::new(config))?;
    conn.complete_io(&mut stream)?;

    conn.writer()
        .write_all(b"Hello from the server")?;
    conn.complete_io(&mut stream)?;
    let mut buf = [0; 64];
    let len = conn
        .reader()
        .read(&mut buf)?;
    println!("Received message from client: {:?}", &buf[..len]);

    Ok(())
}
