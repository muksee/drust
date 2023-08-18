use std::{
    cell::RefCell,
    sync::{
        Arc,
        Mutex,
    },
    thread,
    vec,
};

struct MyStruct {
    mux: Mutex<RefCell<i32>>,
}

fn main() {
    let shared = Arc::new(MyStruct {
        mux: Mutex::new(RefCell::new(100)),
    });

    let mut hs = vec![];

    for i in 0..10 {
        let shared1 = Arc::clone(&shared);
        hs.push(thread::spawn(move || {
            let mg = shared1
                .mux
                .lock()
                .unwrap();
            println!(
                "thread {:?}: orgin {}",
                thread::current().id(),
                mg.borrow()
            );
            *mg.borrow_mut() = i;
        }));
    }

    for h in hs {
        _ = h.join();
    }

    // let _ = env_logger::init();

    // trace!("a trace example");
    // debug!("deboogging");
    // info!("such information");
    // warn!("o_O");
    // error!("boom");
}
