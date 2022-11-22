use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(1, Ordering::SeqCst);
    }
}

#[global_allocator]
static COUNTER: Counter = Counter;

fn main() {
    let mut v = Vec::new();

    for i in 0..10 {
        v.push(Box::new(i));
        println!(
            "allocated bytes before main: {}",
            ALLOCATED.load(Ordering::SeqCst)
        );
    }
}
