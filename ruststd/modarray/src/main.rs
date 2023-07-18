use std::array::{
    from_fn,
    from_mut,
    from_ref,
};

fn main() {
    println!("Hello, world!");

    let a = 100i32;
    let a_array = from_ref(&a);
    println!("{a_array:?}");

    let mut b = 100i32;
    let b_array = from_mut(&mut b);
    println!("{b_array:?}");

    let array1 = from_fn::<usize, 8, _>(|i| i * 3);
    let array2: [usize; 8] = from_fn(|i| i * 3);
    println!("{array1:?},{array2:?}");
}
