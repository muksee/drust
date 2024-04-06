use h2::server;
use http::StatusCode;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    loop {
        if let Ok((socket, _peer_addr)) = listener
            .accept()
            .await
        {
            tokio::spawn(async {
                let mut h2 = server::handshake(socket)
                    .await
                    .unwrap();
                while let Some(request) = h2
                    .accept()
                    .await
                {
                    let (request, mut respond) = request.unwrap();
                    println!("Received request: {:?}", request);

                    let response = http::Response::builder()
                        .status(StatusCode::OK)
                        .body(())
                        .unwrap();

                    respond
                        .send_response(response, true)
                        .unwrap();
                }
            });
        }
    }
}
