//! Oneshot Channel
//! - 状态管理：单原子变量。
//! - 发送限制：通过类型系统限定只发一次，首次发送即消耗掉发送器。
//! - 发送器实现：通过Arc来共享通道。

use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        Arc,
        atomic::{
            self,
            Ordering,
        },
    },
    thread,
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: atomic::AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Channel<T> {
    pub fn channel() -> (Sender<T>, Receiver<T>) {
        let channel = Arc::new(Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: atomic::AtomicBool::new(false),
        });

        (
            Sender {
                channel: channel.clone(),
            },
            Receiver {
                channel: channel.clone(),
            },
        )
    }
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }

        self.channel
            .ready
            .store(true, Ordering::Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel
            .ready
            .load(Ordering::Relaxed)
    }
    pub fn receive(self) -> T {
        if !self
            .channel
            .ready
            .swap(false, Ordering::Acquire)
        {
            panic!("no message avaliable");
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message
                    .get_mut()
                    .assume_init_drop();
            }
        }
    }
}
fn main() {
    thread::scope(|s| {
        let (sender, receiver) = Channel::channel();
        let t = thread::current();
        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });
        while !receiver.is_ready() {
            thread::park();
        }
        assert_eq!(receiver.receive(), "hello world!");
    });
}
