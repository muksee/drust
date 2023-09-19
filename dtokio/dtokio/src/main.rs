#![feature(type_name_of_val)]
use std::{
    any::type_name_of_val,
    thread,
    time::Duration,
};
use tokio::runtime::Builder;

fn main() {
    let sb = std::thread::Builder::new().stack_size(1024 * 1024);

    let h = sb
        .spawn(|| {
            test_tokio();
        })
        .unwrap();

    let _ = h.join();

    let a = test_buf();
    println!("{:?}", type_name_of_val(&a));
}

fn test_tokio() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .thread_name("my-custom-name")
        .thread_stack_size(4 * 655360000)
        .build()
        .unwrap();

    runtime.block_on(async {
        // for _ in 0..100 {
        //     let _ = test_buf().await;
        // }
    });
}

async fn test_buf() {
    let buf = [0; 65536];
    // test_buf0(&buf).await;
}
async fn test_buf0(buf: &[u8]) {
    println!("ttt");
}
