use std::thread;

use scoped_tls::scoped_thread_local;

scoped_thread_local!(static FOO: u32);

fn main() {
    thread::spawn(|| {
        FOO.set(&100, || {
            let val = FOO.with(|v| *v);
            assert_eq!(val, 100);

            // set can be called recursively
            FOO.set(&101, || {
                println!(".......{:?}", thread::current());
            });

            // Recursive calls restore the previous value.
            let val = FOO.with(|v| *v);
            assert_eq!(val, 100);
        });
    }).join();
}
