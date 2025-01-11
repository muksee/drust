use hyper::{
    client::conn::Builder,
    server::conn::Http,
    service::service_fn,
    upgrade::Upgraded,
    Body,
    Request,
    Response,
};
use std::net::SocketAddr;
use tokio::net::{
    TcpListener,
    TcpStream,
};

extern crate tokio;

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 8000).into();
    let tcp_listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = tcp_listener
            .accept()
            .await?;

        tokio::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(proxy))
                .await
            {
                eprintln!("Failed to server connection: {:?}", err);
            }
        });
    }
}

async fn proxy(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("req: {:?}", req);

    if hyper::Method::CONNECT == req.method() {
        if let Some(addr) = host_addr(req.uri()) {
            match hyper::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(err) = tunel(upgraded, addr).await {
                        eprintln!("server io error: {:?}", err);
                    }
                }
                Err(err) => {
                    eprintln!("upgrade error: {:?}", err);
                }
            };

            Ok(Response::new(Body::empty()))
        } else {
            eprintln!(
                "CONNECT host is not socket addr: {:?}",
                req.uri()
            );

            let mut resp = Response::new(Body::from(
                "CONNECT must be to a socket address",
            ));

            *resp.status_mut() = hyper::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        let host = req
            .uri()
            .host()
            .expect("uri has no host");

        let port = req
            .uri()
            .port_u16()
            .unwrap_or(80);

        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(addr)
            .await
            .unwrap();

        let (mut sender, conn) = Builder::new()
            .http1_preserve_header_case(true)
            .http1_title_case_headers(true)
            .handshake(stream)
            .await?;

        tokio::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Connection failed: {:?}", err);
            }
        });

        sender
            .send_request(req)
            .await
    }
}

fn host_addr(uri: &hyper::Uri) -> Option<String> {
    uri.authority()
        .map(|auth| auth.to_string())
}

async fn tunel(mut upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
    print!(
        "client wrote {} bytes and received {} bytes",
        from_client, from_server
    );

    Ok(())
}
