use std::{
    thread,
    time::Duration,
};

fn main() {
    use once_cell::sync::Lazy;
    pub static pool: Lazy<rayon::ThreadPool> = Lazy::new(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap()
    });

    thread::sleep(std::time::Duration::from_millis(1000));
    // for i in 0..90 {
    //     pool.spawn(move || {
    //         print!("init: {}, ", i);
    //     });
    // }

    pool.spawn_broadcast(move |e| {
        println!("init: {e:?}");
    });

    thread::sleep(std::time::Duration::from_millis(200));
    thread::sleep(std::time::Duration::from_millis(200));
    let add_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));

    for _i in 0..2 {
        let time_hs = std::time::Instant::now();
        let add_count = add_count.clone();
        pool.spawn(move || {
            let t = time_hs.elapsed();
            println!("current thread : {:?}", pool.current_thread_index());
            add_count.fetch_add(
                t.as_micros() as usize,
                std::sync::atomic::Ordering::Relaxed,
            );
        });
    }

    std::thread::sleep(std::time::Duration::from_millis(2000));
    println!(
        "线程开启平均耗时: {:?} micros",
        add_count.load(std::sync::atomic::Ordering::Relaxed) / 2
    );
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
