#![allow(unused)]

use std::ops::ControlFlow;

fn main() {
    println!("{:?}", test());
}

fn test() -> ControlFlow<i32, i32> {
    ControlFlow::Break(100)
}

fn exec() -> ControlFlow<i32, i32> {
    let r = test()?;
    ControlFlow::Continue(r)
}
