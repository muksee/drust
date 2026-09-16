//! 更新计数的点：
//! Arc::clone
//! Arc::drop
//! Arc::downgrade - 间接通过Weak::clone
//! Weak::clone
//! Weak::drop
//! Weak::upgrade - 直接更新data_ref_count，间接Weak::clone更新alloc_ref_count
//!
//! 设计核心：
//! - 每个Arc都通过内部Weak持有数据，Arc计数等于内部Weak数。在Arc外部的Weak称为独立Weak。
//! - alloc_ref_count跟踪所有Weak(Arc内部Weak+独立Weak)，归零后回收内存资源。
//! - data_ref_count跟踪Arc计数(即内部Weak数)，归零后销毁数据(置为None)，但是不回收内存。
//!   此时Weak 还能访问到数据，但是是None
//!
//! data_ref_count ─── Arc count  ==  inner Weak count            
//!  │                               │                            
//!  └ 0 set data None               +                            
//!                                  │                            
//!                                  │                            
//! alloc_ref_count ─── Weak Count ──┘standalone weak count       
//!  │                                                            
//!  └ 0 dealloc memory
use std::{
    cell::UnsafeCell,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{
        self,
        Ordering,
        fence,
    },
};

struct ArcData<T> {
    // Arc计数。每个Arc中都包含一个Weak。
    data_ref_count: atomic::AtomicUsize,
    // 所有Weak计数：Arc计数(非独立Weak计数) + 独立Weak计数。
    alloc_ref_count: atomic::AtomicUsize,
    data: UnsafeCell<Option<T>>,
}

pub struct Arc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Sync + Send> Send for Weak<T> {}
unsafe impl<T: Sync + Send> Sync for Weak<T> {}

impl<T> Arc<T> {
    pub fn new(data: T) -> Arc<T> {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    alloc_ref_count: atomic::AtomicUsize::new(1),
                    data_ref_count: atomic::AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc
            .weak
            .data()
            .alloc_ref_count
            .load(Ordering::Relaxed)
            == 1
        {
            fence(Ordering::Acquire);
            // 安全性：没有任何东西可以访问 data，因为
            // 仅有一个 Arc，并且我们拥有独占访问权限，
            // 也没有 Weak 指针
            let arcdata = unsafe { arc.weak.ptr.as_mut() };
            let option = arcdata.data.get_mut();
            // 我们知道 data 是仍然可获得的，因为我们
            // 有一个 Arc 去包裹它，因此不会 panic。
            let data = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        // alloc_ref_count++
        arc.weak.clone()
    }
}

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        // 乐观锁：
        // 因为获取到计数、判断为0、更新计数这三个操作之间没有原子性，
        // 在操作的间隙可能计数就被其它人更新过，导致最终的计数更新错误。
        //
        // data_ref_count++
        // alloc_ref_count++，Arc+Weak
        let mut n = self
            .data()
            .data_ref_count
            .load(Ordering::Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n <= usize::MAX / 2);
            if let Err(e) = self
                .data()
                .data_ref_count
                .compare_exchange_weak(
                    n,
                    n + 1,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
            {
                n = e;
                continue;
            }
            return Some(Arc { weak: self.clone() });
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> Clone for Weak<T> {
    // alloc_ref_count++
    fn clone(&self) -> Self {
        if self
            .data()
            .alloc_ref_count
            .fetch_add(1, Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }

        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    // data_ref_count++
    // alloc_ref_count++
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak
            .data()
            .data_ref_count
            .fetch_add(1, Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }
        Arc { weak }
    }
}

impl<T> Drop for Weak<T> {
    // alloc_ref_count--
    fn drop(&mut self) {
        if self
            .data()
            .alloc_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Drop for Arc<T> {
    // data_ref_count--
    // alloc_ref_count--，会自动调用字段weak的drop。
    fn drop(&mut self) {
        if self
            .weak
            .data()
            .data_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);
            let ptr = self.weak.data().data.get();
            // 安全性：data 引用计数是 0，
            // 因此没有任何东西可以访问它。
            unsafe {
                (*ptr) = None;
            }
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
