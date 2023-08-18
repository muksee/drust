#![feature(generator_trait, generators)]

use std::{
    ops::{
        Generator,
        GeneratorState,
    },
    pin::Pin,
    sync::Arc,
    time::Duration, thread,
};

use parking_lot::RwLock;

#[tokio::main]
async fn main() {
    let lock = Arc::new(RwLock::new(100i32));

    // // let _ = tokio::spawn(async move {
    // //     main0(lock).await;
    // // })
    // // .await;

    // // main0(lock);

    main3(lock)
}

async fn main0(lock: Arc<RwLock<i32>>) {
    let guard = lock.read();
    {
        drop(guard);
    }
    tokio::time::sleep(Duration::from_secs(3)).await;
}

// fn main2() {
//     let a = String::from("abc");
//     drop(a);
//     println!("{}", a);
// }

fn main3(lock: Arc<RwLock<i32>>) {
    let generator = ||{
        yield 2;
        let guard = lock.read();
        drop(guard);
        yield 1;
        "foo"
    };

    thread::spawn(move || {
        let a = generator;
    });
}
