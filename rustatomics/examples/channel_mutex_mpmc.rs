//!
//! 逻辑：
//! - 队列使用互斥锁Mutex进行保护。
//! - 队列写入消息后通过Condvar发送通知给接收者。

use std::{
    collections::VecDeque,
    sync,
    thread,
};

pub struct Channel<T> {
    queue: sync::Mutex<VecDeque<T>>,
    item_ready: sync::Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: sync::Mutex::new(VecDeque::new()),
            item_ready: sync::Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue
            .lock()
            .unwrap()
            .push_back(message);

        self.item_ready.notify_one();
    }

    pub fn recv(&self) -> T {
        let mut lock = self.queue.lock().unwrap();
        loop {
            if let Some(message) = lock.pop_front() {
                return message;
            }

            lock = self.item_ready.wait(lock).unwrap();
        }
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let channel = Channel::new();

    thread::scope(|s| {
        s.spawn(|| {
            loop {
                let message = channel.recv();
                println!(
                    "thread {:?}, recv: {}",
                    thread::current().id(),
                    message
                );
            }
        });
        s.spawn(|| {
            loop {
                let message = channel.recv();
                println!(
                    "thread {:?}, recv: {}",
                    thread::current().id(),
                    message
                );
            }
        });
        s.spawn(|| {
            for i in 0..10 {
                channel.send(format!("Hello {i}"));
            }
        });
    });
}
