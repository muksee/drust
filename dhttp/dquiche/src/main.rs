use core::panic;
use std::{
    collections::HashMap,
    net,
};

use mio::{
    Events,
    Interest,
    Poll,
};
use quiche::PROTOCOL_VERSION;
use ring::rand::SystemRandom;

use log::*;

const MAX_DATAGRAM_SIZE: usize = 1350;

fn main() {
    env_logger::init();

    let mut buf = [0; 65535];
    let mut out = [0; MAX_DATAGRAM_SIZE];

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    let mut socket = mio::net::UdpSocket::bind(
        "0.0.0.0:4433"
            .parse()
            .unwrap(),
    )
    .unwrap();
    poll.registry()
        .register(&mut socket, mio::Token(0), Interest::READABLE)
        .unwrap();

    let mut config = quiche::Config::new(PROTOCOL_VERSION).unwrap();

    config
        .set_application_protos(&[
            b"hq-interop",
            b"hq-29",
            b"hq-28",
            b"hq-27",
            b"http/0.9",
        ])
        .unwrap();

    config.set_max_idle_timeout(5000);
    config.set_max_recv_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_max_send_udp_payload_size(MAX_DATAGRAM_SIZE);
    config.set_initial_max_data(10_000_000);
    config.set_initial_max_stream_data_bidi_local(1_000_000);
    config.set_initial_max_stream_data_bidi_remote(1_000_000);
    config.set_initial_max_stream_data_uni(1_000_000);
    config.set_initial_max_streams_bidi(100);
    config.set_initial_max_streams_uni(100);
    config.set_disable_active_migration(true);
    config.enable_early_data();

    let rng = SystemRandom::new();
    let conn_id_seed =
        ring::hmac::Key::generate(ring::hmac::HMAC_SHA256, &rng).unwrap();

    let mut clients = ClientMap::new();
    let local_addr = socket
        .local_addr()
        .unwrap();

    loop {
        let timeout = clients
            .values()
            .filter_map(|c| {
                c.conn
                    .timeout()
            })
            .min();

        poll.poll(&mut events, timeout)
            .unwrap();

        'read: loop {
            if events.is_empty() {
                println!("timed out");
                clients
                    .values_mut()
                    .for_each(|c| {
                        c.conn
                            .on_timeout()
                    });
                break 'read;
            }

            let (len, from) = match socket.recv_from(&mut buf) {
                Ok(v) => v,
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        println!("recv() wrouldblock");
                        break 'read;
                    }

                    panic!("recv() failed: {:?}", e);
                }
            };

            println!("got {} bytes", len);

            let pkt_buf = &mut buf[..len];

            let hdr = match quiche::Header::from_slice(
                pkt_buf,
                quiche::MAX_CONN_ID_LEN,
            ) {
                Ok(v) => v,
                Err(e) => {
                    println!("Parsing packet header failed: {:?}", e);
                    continue 'read;
                }
            };

            println!("got packet {:?}", hdr);

            let conn_id = ring::hmac::sign(&conn_id_seed, &hdr.dcid);
            let conn_id = &conn_id.as_ref()[..quiche::MAX_CONN_ID_LEN];
            let conn_id = conn_id
                .to_vec()
                .into();

            let client = if !clients.contains_key(&hdr.dcid)
                && !clients.contains_key(&conn_id)
            {
                if hdr.ty != quiche::Type::Initial {
                    println!("Packet not Initial");
                    continue 'read;
                }

                if !quiche::version_is_supported(hdr.version) {
                    println!("Doing version negotiation");

                    let len =
                        quiche::negotiate_version(&hdr.scid, &hdr.dcid, &mut out)
                            .unwrap();
                    let out = &out[..len];

                    if let Err(e) = socket.send_to(out, from) {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            println!("send() would block");
                            break;
                        }

                        panic!("send() failed: {:?}", e);
                    }

                    continue 'read;
                }

                let mut scid = [0; quiche::MAX_CONN_ID_LEN];
                scid.copy_from_slice(&conn_id);

                let scid = quiche::ConnectionId::from_ref(&scid);
                let token = hdr
                    .token
                    .as_ref()
                    .unwrap();

                if token.is_empty() {
                    println!("Doing stateless retry");
                    let new_token = mint_token(&hdr, &from);

                    let len = quiche::retry(
                        &hdr.scid,
                        &hdr.dcid,
                        &scid,
                        &new_token,
                        hdr.version,
                        &mut out,
                    )
                    .unwrap();

                    let out = &out[..len];
                    if let Err(e) = socket.send_to(out, from) {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            println!("send() wouldblock");
                            break;
                        }

                        panic!("send() failed: {:?}", e);
                    }

                    continue 'read;
                }

                let odcid = validate_token(&from, token);

                if odcid.is_none() {
                    println!("Invalid address validation token");
                    continue 'read;
                }

                if scid.len()
                    != hdr
                        .dcid
                        .len()
                {
                    println!("Invalid destination connection ID");
                    continue 'read;
                }

                let scid = hdr
                    .dcid
                    .clone();
                println!("New connection: dcid={:?} scid={:?}", hdr.dcid, scid);

                let conn = quiche::accept(
                    &scid,
                    odcid.as_ref(),
                    local_addr,
                    from,
                    &mut config,
                )
                .unwrap();

                let client = Client {
                    conn,
                    partial_responses: HashMap::new(),
                };

                clients.insert(scid.clone(), client);
                clients
                    .get_mut(&scid)
                    .unwrap()
            } else {
                match clients.get_mut(&hdr.dcid) {
                    Some(v) => v,
                    None => clients
                        .get_mut(&conn_id)
                        .unwrap(),
                }
            };

            let recv_info = quiche::RecvInfo {
                to: socket
                    .local_addr()
                    .unwrap(),
                from,
            };

            let read = match client
                .conn
                .recv(pkt_buf, recv_info)
            {
                Ok(v) => v,
                Err(e) => {
                    error!(
                        "{} recv failed: {:?}",
                        client
                            .conn
                            .trace_id(),
                        e
                    );
                    continue 'read;
                }
            };

            debug!(
                "{} processed {} bytes",
                client
                    .conn
                    .trace_id(),
                read
            );

            if client
                .conn
                .is_in_early_data()
                || client
                    .conn
                    .is_established()
            {
                for stream_id in client
                    .conn
                    .writable()
                {
                    handle_writable(client, stream_id);
                }

                for s in client
                    .conn
                    .readable()
                {
                    while let Ok((read, fin)) = client
                        .conn
                        .stream_recv(s, &mut buf)
                    {
                        debug!(
                            "{} received {} bytes",
                            client
                                .conn
                                .trace_id(),
                            read
                        );

                        let stream_buf = &buf[..read];

                        debug!(
                            "{} stream {} has {} bytes (fin? {})",
                            client
                                .conn
                                .trace_id(),
                            s,
                            stream_buf.len(),
                            fin
                        );
                        handle_stream(client, s, stream_buf, "examples/root");
                    }
                }
            }
        }
    }
}

type ClientMap = HashMap<quiche::ConnectionId<'static>, Client>;

struct PartialResponse {
    body: Vec<u8>,
    written: usize,
}

struct Client {
    conn: quiche::Connection,
    partial_responses: HashMap<u64, PartialResponse>,
}

fn handle_writable(client: &mut Client, stream_id: u64) {
    let conn = &mut client.conn;
    debug!("{} stream {} is writable", conn.trace_id(), stream_id);

    if !client
        .partial_responses
        .contains_key(&stream_id)
    {
        return;
    }

    let resp = client
        .partial_responses
        .get_mut(&stream_id)
        .unwrap();
    let body = &resp.body[resp.written..];

    let written = match conn.stream_send(stream_id, body, true) {
        Ok(v) => v,
        Err(quiche::Error::Done) => 0,
        Err(e) => {
            client
                .partial_responses
                .remove(&stream_id);
            error!("{} stream send failed {:?}", conn.trace_id(), e);
            return;
        }
    };

    resp.written += written;

    if resp.written
        == resp
            .body
            .len()
    {
        client
            .partial_responses
            .remove(&stream_id);
    }
}

fn mint_token(hdr: &quiche::Header, src: &net::SocketAddr) -> Vec<u8> {
    let mut token = Vec::new();
    token.extend_from_slice(b"quiche");

    let addr = match src.ip() {
        std::net::IpAddr::V4(a) => a
            .octets()
            .to_vec(),
        std::net::IpAddr::V6(a) => a
            .octets()
            .to_vec(),
    };

    token.extend_from_slice(&addr);
    token.extend_from_slice(&hdr.dcid);

    token
}

fn validate_token<'a>(
    src: &net::SocketAddr,
    token: &'a [u8],
) -> Option<quiche::ConnectionId<'a>> {
    if token.len() < 6 {
        return None;
    }

    if &token[..6] != b"quiche" {
        return None;
    }

    let token = &token[6..];
    let addr = match src.ip() {
        std::net::IpAddr::V4(a) => a
            .octets()
            .to_vec(),
        std::net::IpAddr::V6(a) => a
            .octets()
            .to_vec(),
    };

    if token.len() < addr.len() || &token[..addr.len()] != addr.as_slice() {
        return None;
    }

    Some(quiche::ConnectionId::from_ref(&token[addr.len()..]))
}

fn handle_stream(client: &mut Client, stream_id: u64, buf: &[u8], root: &str) {
    let conn = &mut client.conn;

    if buf.len() > 4 && &buf[..4] == b"GET" {
        let uri = &buf[4..buf.len()];
        let uri = String::from_utf8(uri.to_vec()).unwrap();
        let uri = String::from(
            uri.lines()
                .next()
                .unwrap(),
        );
        let uri = std::path::Path::new(&uri);
        let mut path = std::path::PathBuf::from(root);
    }
}
