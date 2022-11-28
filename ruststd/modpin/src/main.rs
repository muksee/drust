use std::marker::PhantomPinned;
use std::mem;
use std::pin;
use std::pin::Pin;
use std::ptr;

#[derive(Debug)]
struct SelfType {
    x: String,
    y: *const String,
    // _p: PhantomPinned,
}

impl SelfType {
    fn new(x: &str) -> Self {
        let mut this = SelfType {
            x: String::from(x),
            y: ptr::null(),
            // _p: PhantomPinned,
        };
        this.y = &this.x;

        this
    }
}

fn main() {
    let mut st1 = SelfType::new("st-1");
    let mut p1 = unsafe { Pin::new_unchecked(&mut st1) };
    let mut st2 = SelfType::new("st-2");
    let mut p2 = unsafe { Pin::new_unchecked(&mut st2) };

    // println!(
    //     "st1 = {st1:?} st1.y={:?}, st2 = {st2:?}, st2.y={:?}",
    //     unsafe { &*st1.y },
    //     unsafe { &*st2.y }
    // );

    println!("AFTER SWAP:");

    mem::swap(p1.get_mut(), p2.get_mut());
    println!(
        "st1 = {st1:?} st1.y={:?}, st2 = {st2:?}, st2.y={:?}",
        unsafe { &*st1.y },
        unsafe { &*st2.y }
    );
}
