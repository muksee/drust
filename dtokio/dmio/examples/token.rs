use core::panic;
use std::{collections::HashMap, io::Write, thread};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

use std::io::{self, Read};

const MAX_SOCKETS: usize = 32;
const LISTENER: Token = Token(1024);

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut sockets = HashMap::new();

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);

    let mut next_socket_index = 0;

    let mut listener = TcpListener::bind("127.0.0.1:0".parse()?)?;
    poll.registry()
        .register(&mut listener, LISTENER, Interest::READABLE)?;

    let mut buf = [0u8; 256];

    let addr = listener.local_addr()?;
    thread::spawn(move || {
        use std::net::TcpStream;
        for i in 0..MAX_SOCKETS + 1 {
            let mut client = TcpStream::connect(addr).unwrap();
            let _ = client.write(&[i as u8]);
        }
    });

    loop {
        poll.poll(&mut events, None)?;

        for event in &events {
            match event.token() {
                LISTENER => loop {
                    match listener.accept() {
                        Ok((mut socket, _)) => {
                            // if next_socket_index == MAX_SOCKETS {
                            //     return Ok(());
                            // }

                            let token = Token(next_socket_index);
                            next_socket_index += 1;

                            poll.registry()
                                .register(&mut socket, token, Interest::READABLE)?;

                            sockets.insert(token, socket);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("WouldBlock");
                            break;
                        }
                        e => {
                            panic!("err={:?}", e);
                        }
                    }
                },
                token => loop {
                    match sockets.get_mut(&token).unwrap().read(&mut buf) {
                        Ok(0) => {
                            sockets.remove(&token);
                            println!("socket:{:?}, shutdown", token);
                            break;
                        }
                        Ok(n) => {
                            println!("socket:{:?}, read={:?}", token, &buf[0..n]);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            println!("socket:{:?}, WouldBlock", token);
                            break;
                        }
                        e => panic!("err={:?}", e),
                    }
                },
            }
        }
    }
}
