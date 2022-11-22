use std::sync::atomic::Ordering::{Acquire, SeqCst};
use std::{alloc::GlobalAlloc, cell::UnsafeCell, ptr::null_mut, sync::atomic::AtomicUsize};

const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;

#[repr(C, align(4096))]
struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize,
}

unsafe impl Sync for SimpleAllocator {}
unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;

        let updated = self
            .remaining
            .fetch_update(SeqCst, SeqCst, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            });

        if updated.is_err() {
            return null_mut();
        }

        self.arena.get().cast::<u8>().add(allocated)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: std::alloc::Layout) {}
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};

fn main() {
    for _ in 0..10 {
        let s = "xxxxxxxxxx".to_string();
        println!("{s}");
        let currently = ALLOCATOR.remaining.load(Acquire);
        println!("allocated so far: {}", ARENA_SIZE - currently);
    }

    println!("{:?}", ALLOCATOR.arena);
}
