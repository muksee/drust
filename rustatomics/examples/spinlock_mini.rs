use std::{
    hint,
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    thread,
};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> SpinLock {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self
            .locked
            .swap(true, Ordering::Acquire)
        {
            hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked
            .store(false, Ordering::Release);
    }
}

impl Default for SpinLock {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let lock = SpinLock::new();

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..10 {
                lock.lock();
                println!("Thread 1, {}", i);
                lock.unlock();
            }
        });
        s.spawn(|| {
            for i in 0..10 {
                lock.lock();
                println!("Thread 2, {}", i);
            }
        });
    });
}
