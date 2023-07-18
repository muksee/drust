#![feature(new_uninit)]

//! Box和Rc等智能指针为什么那么多关联方法: 为了防止和指针解引用后内部值的方法冲突

use std::{
    alloc::{
        alloc,
        Layout,
    },
    any::Any,
};

fn main() {
    let mut five = Box::<u32>::new_uninit();

    let five = unsafe {
        five.as_mut_ptr()
            .write(100);
        five.assume_init()
    };

    let big_box = Box::<[usize; 8]>::new_uninit();
    let mut array = [0; 8];
    for (i, e) in array
        .iter_mut()
        .enumerate()
    {
        *e = i;
    }

    let big_box = Box::write(big_box, array);
    println!("{big_box:p}");

    let b: Box<dyn Any> = Box::new(100i32);
    if let Ok(inner) = b.downcast::<i32>() {
        println!("downcast: {inner}");
    }

    let b1 = Box::new(100i32);
    let ptr = Box::into_raw(b1);
    let b2 = unsafe { Box::from_raw(ptr) };

    unsafe {
        let ptr = alloc(Layout::new::<i32>()) as *mut i32;
        ptr.write(200);
        let b = Box::from_raw(ptr);
    }

    let b1 = Box::new(100i32);
    let c1 = *b1;

    println!("{},{}", b1, c1);
}
