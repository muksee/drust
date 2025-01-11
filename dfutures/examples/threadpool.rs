use crossbeam::channel;
use std::{
    thread,
    time::Instant,
};

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

/// 简易线程池
pub struct ThreadPoolLite {
    jobs: channel::Sender<Box<dyn FnOnce() + Send + 'static>>,
    _handles: Vec<thread::JoinHandle<()>>,
}

impl ThreadPoolLite {
    pub fn new(size: usize) -> ThreadPoolLite {
        let (tx, rx) = channel::unbounded();
        let mut _handles = Vec::with_capacity(size);

        for i in 0..size {
            let rx: channel::Receiver<Box<dyn FnOnce() + Send>> = rx.clone();
            let handle = thread::spawn(move || {
                // println!("ThreadPoolLite init: {i}");
                while let Ok(job) = rx.recv() {
                    job();
                }
            });
            _handles.push(handle);
        }

        ThreadPoolLite { jobs: tx, _handles }
    }

    pub fn spawn<F, T>(&self, f: F) -> channel::Receiver<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = channel::bounded(1);
        let now = Instant::now();
        let job = Box::new(move || {
            let result = f();
            println!("pre execute epalsed: {:?}", now.elapsed());
            if let Err(e) = tx.send(result) {
                // error!("ThreadPool::execute: {}", e);
            }
        });

        if let Err(e) = self
            .jobs
            .send(job)
        {
            println!("ThreadPool::execute: {}", e);
        }
        // println!("send job epalsed: {:?}", now.elapsed());

        rx
    }
}

fn main() {
    use once_cell::sync::Lazy;
    pub static pool: Lazy<ThreadPoolLite> =
        Lazy::new(|| ThreadPoolLite::new(6));
    thread::sleep(std::time::Duration::from_millis(5000));
    let rx = pool.spawn(|| {
        println!("hello1");
        1
    });
    let rx2 = pool.spawn(|| {
        println!("hello2");
        2
    });
    let rx3 = pool.spawn(|| {
        // thread::sleep(std::time::Duration::from_millis(20));
        println!("hello3"); 
        3
    });
    let rx4 = pool.spawn(|| {
        println!("hello4");
        4
    });
    let rx5 = pool.spawn(|| {
        // thread::sleep(std::time::Duration::from_millis(25));
        println!("hello5");
        5
    });
    let rx6 = pool.spawn(|| {
        println!("hello6");
        6
    });
    println!(
        "result: {:?},{:?},{:?},{:?},{:?},{:?}",
        rx.recv(),
        rx2.recv(),
        rx3.recv(),
        rx4.recv(),
        rx5.recv(),
        rx6.recv()
    );
    let r = pool.spawn(|| {
        println!("hello6");
        1
    });
    r.recv();
}
