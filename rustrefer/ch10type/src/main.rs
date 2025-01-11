#[allow(unused)]
fn main() {
    let mut v = String::from("Hello");

    let mut f = || {
        v.push_str("world");
        v
    };

    let s = f();
}
