use std::{
    sync::OnceLock,
};

static oc: OnceLock<i32> = OnceLock::new();

fn main() {
    println!("Hello, world!");

    let value = oc.get_or_init(|| 200);

    println!("{:?}", value);
}
