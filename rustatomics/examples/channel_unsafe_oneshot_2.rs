//! Oneshot Channel
//! - 状态管理：单原子变量。
//! - 发送限制：运行时判定，使用复合的状态字段。

use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        self,
        Ordering,
    },
    thread,
};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct Channel<T> {
    state: atomic::AtomicU8,
    message: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            state: atomic::AtomicU8::new(EMPTY),
            message: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(
                EMPTY,
                WRITING,
                Ordering::Relaxed,
                Ordering::Relaxed,
            )
            .is_err()
        {
            panic!("can't send more than one message");
        }

        unsafe {
            (*self.message.get()).write(message);
        }

        self.state
            .store(READY, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        if self
            .state
            .compare_exchange(
                READY,
                READING,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
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
        if *self.state.get_mut() == READY {
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
