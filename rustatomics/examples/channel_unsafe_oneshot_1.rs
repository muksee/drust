//! Oneshot Channel
//! - 状态管理：双原子变量。写入状态ready、限制一次写入状态in_use。
//! - 发送限制：运行时判定，使用单独的原子状态。

use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        self,
        Ordering,
    },
    thread,
};

/// onshot channel
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: atomic::AtomicBool,
    // 标记通道已经发送过一次了, 由于是onshot, 不能再发送
    in_use: atomic::AtomicBool,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: atomic::AtomicBool::new(false),
            in_use: atomic::AtomicBool::new(false),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .in_use
            .swap(true, Ordering::Relaxed)
        {
            panic!("can't send more than one message");
        }
        unsafe {
            (*self.message.get()).write(message);
        }
        self.ready
            .store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn receive(&self) -> T {
        if !self
            .ready
            .swap(false, Ordering::Acquire)
        {
            panic!("no message avaliable");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
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
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        assert_eq!(channel.receive(), "hello world!");
    });
}
