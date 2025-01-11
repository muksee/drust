/*
EPOLL:
https://man7.org/linux/man-pages/man7/epoll.7.html
MIO:
https://docs.rs/mio
https://github.com/tokio-rs/mio
 */

use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let addr = "127.0.0.1:0".parse()?;
    let server = TcpListener::bind(addr)?;
    let mut stream = TcpStream::connect(server.local_addr()?)?;

    // EPOLL: epoll_crate(int size)
    let mut poll = Poll::new()?;
    // EPOLL: 已发生的事件缓存
    let mut events = Events::with_capacity(1024);

    // EPOLL: epoll_ctl(epfd, int op, int fd, epoll_event *event)
    poll.registry()
        .register(
            &mut stream,
            Token(0),
            Interest::READABLE | Interest::WRITABLE,
        )?;

    println!("{}", events.is_empty());

    loop {
        // EPOLL: int epoll_wait(int epfd, struct epoll_event *events, int maxevents, int timeout)
        poll.poll(&mut events, None)?;

        for event in &events {
            if event.token() == Token(0) && event.is_writable() {
                println!("writable");
                return Ok(());
            }
        }
    }
}
