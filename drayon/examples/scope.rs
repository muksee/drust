use std::{
    thread,
    time::Duration,
};

use rayon::{
    current_thread_index,
    ThreadPoolBuilder,
};

fn main() {
    println!("main ---> {:?}", thread::current().id());
    {
        let pool = ThreadPoolBuilder::new()
            .thread_name(|x| format!("my-thread-{x}"))
            .num_threads(3)
            .start_handler(|x| {
                println!("start {x},{:?}", thread::current().id())
            })
            .exit_handler(|x| println!("stop {x},{:?}", thread::current().id()))
            .build()
            .unwrap();

        thread::sleep(Duration::from_secs(2));

        pool.install(|| {
            println!(
                "i am in install, {:?},{:?}",
                current_thread_index(),
                thread::current().id()
            )
        });
    }

    thread::sleep(Duration::from_secs(2));
}
