use std::{
    alloc::{
        GlobalAlloc,
        Layout,
        System,
    },
    sync::atomic::{
        AtomicUsize,
        Ordering,
    },
};

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
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static COUNTER: Counter = Counter;

fn main() {
    let mut v = Vec::new();

    println!(
        "--> allocated bytes before main: {}",
        ALLOCATED.load(Ordering::SeqCst)
    );
    for i in 0..10 {
        v.push(Box::new(i));
        println!(
            "allocated bytes before main: {}",
            ALLOCATED.load(Ordering::SeqCst)
        );
    }

    drop(v);

    println!(
        "--> allocated bytes before main: {}",
        ALLOCATED.load(Ordering::SeqCst)
    );
}
