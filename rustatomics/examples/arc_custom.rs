use std::{
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{
        self,
        Ordering,
        fence,
    },
};

struct ArcData<T> {
    ref_count: atomic::AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Sync for Arc<T> {}
unsafe impl<T: Send + Sync> Send for Arc<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Self {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: atomic::AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self
            .data()
            .ref_count
            .load(Ordering::Relaxed)
            == 1
        {
            // 同步结点：ref_count
            // 同步事件：其它所有线程在drop之前的写操作 HB 本线程内存回收操作
            fence(Ordering::Acquire);
            unsafe { Some(&mut self.ptr.as_mut().data) }
        } else {
            None
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self
            .data()
            .ref_count
            .fetch_add(1, Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }

        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self
            .data()
            .ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            // 同步结点：ref_count
            // 同步事件：其它所有线程在drop之前的写操作 HB 本线程内存回收操作
            fence(Ordering::Acquire);
            unsafe { drop(Box::from_raw(self.ptr.as_ptr())) }
        }
    }
}

fn main() {
    test();
}

pub fn test() {
    static NUM_DROPS: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
        fn drop(&mut self) {
            NUM_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    // 创建两个 Arc，共享一个对象，包含一个字符串
    // 和一个 DetectDrop，以当它被丢弃时去检测。
    let x = Arc::new(("hello", DetectDrop));
    let y = x.clone();

    // 发送 x 到另一个线程，并在那里使用它。
    let t = std::thread::spawn(move || {
        assert_eq!(x.0, "hello");
    });

    // 这是并行的，y 应该仍然在这里可用。
    assert_eq!(y.0, "hello");

    // 等待线程完成。
    t.join().unwrap();

    // Arc，x 现在应该被丢弃。
    // 我们仍然有 y，因此对象仍然还没有被丢弃。
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

    // 丢弃剩余的 `Arc`。
    drop(y);

    // 现在，`y` 也被丢弃，
    // 对象应该也被丢弃。
    assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
}
