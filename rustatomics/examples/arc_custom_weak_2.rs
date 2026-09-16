//!
//! Ard和Weak
//!
//! 计数：
//! 1.data_ref_count，Arc计数，归零时销毁T并让alloc_ref_count--。
//! 2.alloc_ref_count，Weak数量+1(如果有任意Arc存活)，归零时销毁ArcData。
//!
//! 核心思想：
//! Weak计数除了记录Weak数量，还记录是否还有Arc存活(+1)，即如果有Arc存活，
//! Weak的数量总是多1个，最后一个Arc消失时Weak计数减一。当Weak计数到0时，
//! 说明既没有Arc也没有Weak了，可以将ArcData销毁了。
use std::{
    cell::UnsafeCell,
    mem::ManuallyDrop,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{
        self,
        Ordering,
        fence,
    },
    usize,
};

struct ArcData<T> {
    data_ref_count: atomic::AtomicUsize,
    alloc_ref_count: atomic::AtomicUsize,
    data: UnsafeCell<ManuallyDrop<T>>,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Arc<T> {}
unsafe impl<T: Sync + Send> Sync for Arc<T> {}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}
unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                data_ref_count: atomic::AtomicUsize::new(1),
                alloc_ref_count: atomic::AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return None;
        }

        let is_unique = self
            .data()
            .data_ref_count
            .load(Ordering::Relaxed)
            == 1;

        self.data()
            .alloc_ref_count
            .store(1, Ordering::Release);

        if !is_unique {
            return None;
        }

        fence(Ordering::Acquire);
        unsafe { Some(&mut *self.data().data.get()) }
    }

    pub fn downgrade(&self) -> Weak<T> {
        let mut n = self
            .data()
            .alloc_ref_count
            .load(Ordering::Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = self
                    .data()
                    .alloc_ref_count
                    .load(Ordering::Relaxed);
                continue;
            }

            if let Err(e) = self
                .data()
                .alloc_ref_count
                .compare_exchange_weak(
                    n,
                    n + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                )
            {
                n = e;
                continue;
            }

            return Weak { ptr: self.ptr };
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data().data.get() }
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self
            .data()
            .data_ref_count
            .load(Ordering::Relaxed);

        loop {
            if n == 0 {
                return None;
            }
            if let Err(e) = self
                .data()
                .data_ref_count
                .compare_exchange(n, n + 1, Ordering::Relaxed, Ordering::Relaxed)
            {
                n += e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self
            .data()
            .alloc_ref_count
            .load(Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }

        Weak { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self
            .data()
            .alloc_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);

            // 既无Arc实例也无Weak实例，销毁ArcData。(T没销毁?)。
            drop(Box::from(self.ptr.as_ptr()));
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self
            .data()
            .data_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);

            // 销毁T，此时ArcData依然存在，但是内部T值已经没了。
            // 因此Weak实例也读不到T的值了。
            unsafe {
                ManuallyDrop::drop(&mut *self.data().data.get());
            }

            // 导致的结果是Weak计数alloc_ref_count--。
            // (Weak计数如果减到0会触发销毁ArcData)
            drop(Weak { ptr: self.ptr });
        }
    }
}

fn main() {
    test();
}

fn test() {
    static NUM_DROPS: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // 创建一个 Arc，同时也创建两个 weak 指针。
    let x = Arc::new(("hello", DetectDrop));
    let y = Arc::downgrade(&x);
    let z = Arc::downgrade(&x);

    let t = std::thread::spawn(move || {
        // 此刻，Weak 指针应该被升级。
        let y = y.upgrade().unwrap();
        assert_eq!(y.0, "hello");
    });
    assert_eq!(x.0, "hello");
    t.join().unwrap();

    // data 仍然不应该被丢弃，
    // 并且 weak 指针应该被升级。
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);
    assert!(z.upgrade().is_some());

    drop(x);

    // 现在，data 已经被丢弃，并且
    // weak 指针应该不再被升级。
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    assert!(z.upgrade().is_none());
}
