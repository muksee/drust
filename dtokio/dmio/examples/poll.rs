use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let addr = "127.0.0.1:0".parse()?;
    let server = TcpListener::bind(addr)?;

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);

    let mut stream = TcpStream::connect(server.local_addr()?)?;

    poll.registry().register(
        &mut stream,
        Token(0),
        Interest::READABLE | Interest::WRITABLE,
    )?;

    println!("{}",events.is_empty());

    loop {
        poll.poll(&mut events, None)?;

        for event in &events {
            if event.token() == Token(0) && event.is_writable() {
                println!("writable");
                return Ok(());
            }
        }
    }
}
