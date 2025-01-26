use smol::{
    io,
    net,
    prelude::*,
    Unblock,
};

fn main() {
    smol::block_on(async {
        let mut stream = net::TcpStream::connect("httpbin.org:80")
            .await
            .unwrap();
        let req =
            b"GET / HTTP/1.1\r\nHost: httpbin.org\r\nConnection: close\r\n\r\n";
        stream.write_all(req).await.unwrap();

        let mut stdout = Unblock::new(std::io::stdout());
        io::copy(stream, &mut stdout)
            .await
            .unwrap();
    });

    // smol::spawn(async { 1 + 2 });
}
