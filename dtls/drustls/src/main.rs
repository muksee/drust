use std::cell::{
    Ref,
    RefCell,
};

fn main() {
    thread_local! {
        pub static STATE: RefCell<i32> = RefCell::new(100);
    }

    let state = STATE.with(|s| "123");

    println!("{}", state);
}
