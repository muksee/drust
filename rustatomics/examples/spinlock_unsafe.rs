use std::{
    cell::UnsafeCell,
    hint,
    ops::{
        Deref,
        DerefMut,
    },
    sync::atomic::{
        self,
        Ordering,
    },
    thread,
};

/// 锁资源守卫
///
/// 作用：
/// - 1.看守锁的生效范围，守卫失效时自动解锁，不需要显式的unlock方法了。
/// - 2.通过守卫对锁中值进行访问和编辑。
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .locked
            .store(false, Ordering::Release);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

pub struct SpinLock<T> {
    locked: atomic::AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub fn new(value: T) -> Self {
        SpinLock {
            locked: atomic::AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&'_ self) -> Guard<'_, T> {
        while self
            .locked
            .swap(true, Ordering::Acquire)
        {
            hint::spin_loop();
        }
        Guard { lock: self }
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

fn main() {
    let lock = SpinLock::new(vec![]);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..3 {
                lock.lock().push(i);
            }
        });
        s.spawn(|| {
            for i in 0..3 {
                let mut guard = lock.lock();
                guard.push(i);
                guard.push(i);
            }
        });
    });

    let guard = lock.lock();
    println!("{:?}", guard.as_slice());
}
