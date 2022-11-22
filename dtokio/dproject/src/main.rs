use pin_project_lite::pin_project;
use std::pin;

fn main() {
    let mut s = Struct {
        pinned: 10i32,
        unpinned: 100i64,
    };

    let p = pin::Pin::new(&mut s);
    p.method();
}

pin_project! {
    struct Struct<T,U> {
        #[pin]
        pinned: T,
        unpinned: U,
    }
}

impl Struct<i32, i64> {
    fn method(self: pin::Pin<&mut Self>) {
        let this = self.project();
        let pinned = this.pinned;
        let unpinned = this.unpinned;

        println!("pinned:{:?}, unpinned:{:?}", pinned.get_mut(), unpinned);
    }
}
