//! Oneshot Channel
//! - 状态管理：单原子状态。
//! - 发送限制：通过类型系统判定，是否违反了只发送一次。(引用版，不需堆内存)
//! - 发送器实现：接收方阻塞，receiver必须位于当前线程，不能移动到其它线程。

use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::{
        self,
        Ordering,
    },
    thread,
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: atomic::AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: thread::Thread,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const T>,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: atomic::AtomicBool::new(false),
        }
    }

    /// 每次使用split等于创建一个全新的通道，需要销毁历史通道，初始一个全新的通道。
    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Sender<'a, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }

        self.channel
            .ready
            .store(true, Ordering::Release);

        self.receiving_thread.unpark();
    }
}

impl<'a, T> Receiver<'a, T> {
    pub fn is_ready(&self) -> bool {
        self.channel
            .ready
            .load(Ordering::Relaxed)
    }
    pub fn receive(self) -> T {
        while !self
            .channel
            .ready
            .swap(false, Ordering::Acquire)
        {
            thread::park();
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
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("hello world!");
        });
        assert_eq!(receiver.receive(), "hello world!");
    });
}
