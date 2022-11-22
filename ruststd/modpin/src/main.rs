use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{self, Rc};
use std::sync::Arc;

#[derive(Debug)]
struct Mock;

impl Mock {
    fn m1(self: &Self) {
        println!("m1 = {:?}", self);
    }

    fn m2(self: Box<Self>) {
        println!("m2 = {:?}", self);
    }

    fn m3(self: Arc<Self>) {
        println!("m3 = {:?}", self);
    }

    fn m4(self: Rc<Self>) {
        println!("m4 = {:?}", self);
    }
    fn m5(self: Pin<&mut Self>) {
        println!("m5 = {:?}", self);
    }
    fn m6(self: Pin<&mut Self>) {
        println!("m6 = {:?}", self);
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let rc = rc::Rc::new(RefCell::new(v));

    let r = Rc::clone(&rc);
    let r2 = Rc::clone(&rc);

    let rc_raw = Rc::into_raw(rc);
    unsafe {
        while Rc::strong_count(&r) > 0 {
            Rc::decrement_strong_count(rc_raw);
            println!("{}", Rc::strong_count(&r));
        }
        std::mem::drop(r);
    }

    println!("{r2:?}");
}
