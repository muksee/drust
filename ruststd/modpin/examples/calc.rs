#![allow(unused)]
use std::{mem::swap, pin::pin};

#[derive(Debug)]
struct Foo {
    x: i32,
}

fn main() {
    let s = "111aaa2222bbb333.444.555.55.000ccc-666ddd----777eee88--99ff--111";
    calc(s);
}

const DOT: u8 = '.' as u8;
const LINE: u8 = '-' as u8;

fn calc(s: &str) {
    let (mut start, mut end) = (0, 0);
    let mut pre_dot = false;
    let mut pre_line = false;

    let mut numbers = vec![];

    for c in s.as_bytes() {
        println!(
            "{start:?},{end:?},{pre_line:?},{pre_dot:?},{:?}",
            *c as char
        );
        if c.is_ascii_digit() {
            (end, pre_line) = (end + 1, false);
            continue;
        }

        if *c == DOT && !pre_dot && start < end {
            (end, pre_dot, pre_line) = (end + 1, true, false);
            continue;
        }

        if *c == LINE && start == end {
            (end, pre_line) = (end + 1, true);
            continue;
        }

        if *c == LINE && start < end && pre_line {
            (start, end, pre_line) = (start + 1, end + 1, true);
            continue;
        }

        if start < end && !pre_line {
            numbers.push(&s[start..end]);
            println!("{:?}", &s[start..end]);
        }

        (start, end) = (end + 1, end + 1);
        (pre_line, pre_dot) = (false, false);
    }

    if start < end && !pre_line {
        // println!("{:?}", &s[start..end]);
        numbers.push(&s[start..end]);
    }

    println!("{s:?} -> {numbers:?}");
}
