#![feature(backtrace_frames )]

use std::backtrace::Backtrace;

fn main() {
    println!("Hello, world!");

    let bt = Backtrace::capture();

    println!("{bt}");

    let frames = bt.frames();
    for f in frames {
        println!("{f:?}");
    }
}
