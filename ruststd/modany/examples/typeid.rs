#![feature(type_name_of_val)]

use std::any::{
    type_name,
    type_name_of_val,
    Any,
    TypeId,
};

fn main() {
    let s1 = "Hello String".to_owned();
    assert!(is_string(&s1));

    let s2 = 100i32;
    assert!(!is_string(&s2));

    println!(
        "type_name_of_val: {:?}",
        type_name_of_val(&s1)
    );
    println!(
        "type_name_of_val: {:?}",
        type_name_of_val(&s2)
    );

    println!("type_name: {:?}", type_name::<String>());
    println!("type_name: {:?}", type_name::<i32>());
}

fn is_string<T: ?Sized + Any>(_: &T) -> bool {
    TypeId::of::<T>() == TypeId::of::<String>()
}
