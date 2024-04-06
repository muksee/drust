use std::{fs::File, sync::Arc};

// mio = {version ="0.8.10",features=["os-poll","net"]}
use mio::{
    Events,
    Poll,
    Token,
    Waker,
};

const NOTIFY: Token = Token(1);
fn main() {
    let mut poll = Poll::new().unwrap();
    let stop = Waker::new(poll.registry(), NOTIFY).unwrap();

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        stop.wake()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    let mut events = Events::with_capacity(32);
    println!("poll");
    poll.poll(&mut events, None)
        .unwrap();
    println!("events={:?}", events);

    let f = File::create("abc.txt").unwrap();
}
