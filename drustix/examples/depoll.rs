use std::{
    collections::HashMap,
    io::ErrorKind::WouldBlock,
    net::SocketAddrV4,
    str::{
        FromStr,
        from_utf8,
    },
};

use rustix::{
    buffer::spare_capacity,
    event::epoll::{
        self,
        CreateFlags,
        EventFlags,
    },
    io::{
        ioctl_fionbio,
        read,
        write,
    },
    net::{
        AddressFamily,
        SocketType,
        accept,
        bind,
        listen,
        socket,
    },
};

fn main() {
    println!("Hello epoll");

    // 创建epoll实例
    let epoll_fd = epoll::create(CreateFlags::CLOEXEC).unwrap();

    // 启动监听
    let socket = socket(AddressFamily::INET, SocketType::STREAM, None).unwrap();
    let _ = bind(&socket, &SocketAddrV4::from_str("0.0.0.0:40000").unwrap());
    let _ = listen(&socket, 1);

    // 添加到epoll
    let _ = epoll::add(
        &epoll_fd,
        &socket,
        epoll::EventData::new_u64(1),
        epoll::EventFlags::IN,
    );

    // 下一个可分配的id
    let mut next_id = epoll::EventData::new_u64(2);
    let mut event_list = Vec::with_capacity(4);
    let mut sockets = HashMap::new();

    loop {
        print!("new event waits loop");
        let _ = epoll::wait(&epoll_fd, spare_capacity(&mut event_list), None)
            .unwrap();

        for event in event_list.drain(..) {
            let target = event.data;
            println!("new event processing:{}", target.u64());

            if target.u64() == 1 {
                let conn = accept(&socket).unwrap();
                ioctl_fionbio(&conn, true).unwrap();
                epoll::add(
                    &epoll_fd,
                    &conn,
                    next_id,
                    EventFlags::IN | EventFlags::ET,
                )
                .unwrap();

                write(&conn, b"Hello Rustix\n").unwrap();

                sockets.insert(next_id.u64(), conn);
                println!("client {} get", target.u64());

                next_id = epoll::EventData::new_u64(next_id.u64() + 1);
            } else {
                let conn = sockets.get(&target.u64()).unwrap();
                let mut buffer: [u8; 100] = [b'\0'; 100];
                match read(&conn, &mut buffer) {
                    Ok(len) => {
                        if len == 0 {
                            sockets.remove(&target.u64());
                            println!("client {} leaved", target.u64());
                            continue;
                        } else {
                            let s = from_utf8(&buffer[0..len]).unwrap();
                            println!("client say: {}", s);
                            write(conn, &buffer).unwrap();
                        }
                    }
                    Err(e) => {
                        if e.kind() != WouldBlock {
                            sockets.remove(&target.u64());
                            println!("Error: {}", e.kind());
                        } else {
                            println!("Error: {}", e.kind());
                        }
                    }
                }
            }
        }
    }
}
