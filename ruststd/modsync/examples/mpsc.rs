#![feature(error_reporter)]
use std::{
    error::Report,
    sync::mpsc::{
        channel,
        sync_channel,
    },
    thread,
    time::Duration,
};

fn main() {
    synchannel();
}

fn aschannel() {
    let (tx, rx) = channel();

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("send: {i}");

            let r = tx.send(i);
            if let Err(e) = r {
                let report = Report::new(e)
                    .pretty(true)
                    .show_backtrace(true);
                println!("Err: {report:?}");
            }
        });
    }

    for i in 0..10 {
        let j = rx
            .recv()
            .unwrap();
        println!("recv: {j}");
    }

    thread::sleep(Duration::from_secs(10));
}

fn synchannel() {
    let (tx, rx) = sync_channel(0);

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            println!("send start: {i}");

            let r = tx.send(i);
            if let Err(e) = r {
                let report = Report::new(e)
                    .pretty(true)
                    .show_backtrace(true);
                println!("Err: {report:?}");
            }
            println!("send complete: {i}");
        });
    }
    thread::sleep(Duration::from_secs(3));

    for i in 0..10 {
        let j = rx
            .recv()
            .unwrap();
        println!("recv: {j}");
    }
}
