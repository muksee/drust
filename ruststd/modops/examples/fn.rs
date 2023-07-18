static mut v: Vec<u8> = vec![];

fn main() {
    unsafe {
        v.push(12);
    }

    unsafe {
        let s = v.as_mut_slice();
    }

    let mut a = String::from("Hello, closure");

    let mut f = move || {
        let a = &a;
        println!("{a}");
    };

    f();
    f();
    f();

}
