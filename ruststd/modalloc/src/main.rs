fn say(x: *const ()) {
    println!("{x:?}");
}

fn main() {
    println!("Hello, world!");
    let ptr: fn(*const ()) = say;

    let p = &1 as *const i32;
    ptr(p as *const ());

    let p = &2 as *const i32;
    ptr(p as *const ());
}
