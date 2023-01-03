use std::marker::PhantomPinned;
use std::mem;
use std::pin;
use std::pin::Pin;
use std::ptr;

#[derive(Debug)]
struct SelfType {
    x: String,
    y: *const String,
    _p: PhantomPinned,
}

impl SelfType {
    fn new(x: &str) -> Self {
        let mut this = SelfType {
            x: String::from(x),
            y: ptr::null(),
            _p: PhantomPinned,
        };
        this.y = &this.x;

        this
    }
}

fn main() {
    let mut st = SelfType::new("st");
    let mut boxed = unsafe { Pin::new_unchecked(Box::new(st)) };
}
