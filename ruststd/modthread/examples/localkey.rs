#![feature(type_name_of_val)]

use std::{
    any::type_name_of_val,
    cell::RefCell,
};

thread_local!(
    static Foo: RefCell<u32> = RefCell::new(1);
);

fn main() {
    let a = Foo.with(|f| {
        println!("{}", f.borrow_mut());
        return *f.borrow_mut();
    });

    println!("{}", type_name_of_val(&a));
}
