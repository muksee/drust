use std::{
    sync::{
        Arc,
        Barrier,
    },
    thread::{
        self,
        JoinHandle,
    },
};

fn main() {
    let barrier = Arc::new(Barrier::new(10));

    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(10);

    for i in 0..10 {
        let b = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            println!("before {i}");
            let r = b.wait();
            println!("after {i}, leader: {}", r.is_leader());
        }));
    }

    for h in handles {
        let _ = h.join();
    }
}
