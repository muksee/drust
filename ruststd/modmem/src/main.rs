use std::mem::{MaybeUninit};
use std::ptr::addr_of_mut;
fn main() {
    pub struct Foo {
        name: String,
        list: Vec<u8>,
    }

    let _foo = {
        let mut uninit: MaybeUninit<Foo> = MaybeUninit::uninit();
        let ptr = uninit.as_mut_ptr();

        // Initializing the `name` field
        // Using `write` instead of assignment via `=` to not call `drop` on the
        // old, uninitialized value.
        unsafe {
            addr_of_mut!((*ptr).name).write("Bob".to_string());
        }

        // Initializing the `list` field
        // If there is a panic here, then the `String` in the `name` field leaks.
        unsafe {
            addr_of_mut!((*ptr).list).write(vec![0, 1, 2]);
        }

        // All the fields are initialized, so we call `assume_init` to get an initialized Foo.
        unsafe { uninit.assume_init() }
    };
}