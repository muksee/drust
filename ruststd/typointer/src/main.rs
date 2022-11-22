#![allow(unused)]
use std::ops::Deref;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop person {}", self.name);
    }
}

fn main() {
    use std::mem;
    use std::ptr;

    let v1 = vec![
        Person {
            name: String::from("Lily1"),
            age: 25,
        },
        Person {
            name: String::from("Lucy1"),
            age: 25,
        },
    ];

    let mut v2 = vec![
        Person {
            name: String::from("Lily"),
            age: 26,
        },
        Person {
            name: String::from("Lucy"),
            age: 26,
        },
    ];
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    // 销毁v2内的原有对象防止泄漏
    v2.clear();

    unsafe {
        let v1_ptr = &v1 as *const _;
        let v2_ptr = &mut v2 as *mut _;
        ptr::copy(v1_ptr, v2_ptr, 2);

        // 让所有权系统忘记v1,由于v1和v2引用了相同内存,会导致双重释放
        mem::forget(v1);
    }

    println!("v2 = {:?}", v2);
}
