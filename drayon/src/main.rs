use rayon::prelude::{
    IntoParallelIterator,
    ParallelIterator,
};

use std::{
    cell::RefCell,
    ops::{
        Deref,
        DerefMut,
    },
};

struct A {
    x: String,
}

struct B<'a> {
    y: &'a mut String,
}

fn main() {
    // (0..100)
    //     .into_par_iter()
    //     .for_each(|x| println!("{x:?}"));

    let s = RefCell::new(String::from("ll"));

    // type1
    let mut y = s.borrow_mut();
    y.push_str("kk");

    // type2
    // s.borrow_mut().push_str("kk");

    println!("{}", s.into_inner());
}
