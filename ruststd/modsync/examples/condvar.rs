use std::{
    sync::{
        Arc,
        Condvar,
        Mutex,
    },
    thread,
    time::Duration,
};

fn main() {
    let pair1 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair1);
    let pair3 = Arc::clone(&pair1);

    let mut handles = vec![];
    handles.push(thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut guard = lock
            .lock()
            .unwrap();
        *guard = true;
        println!("notify one!");
        cvar.notify_one();
        thread::sleep(Duration::from_secs(3));
        cvar.notify_all();
        println!("notify all!");
    }));

    handles.push(thread::spawn(move || {
        let (lock, cvar) = &*pair3;
        let guard = lock
            .lock()
            .unwrap();
        let _guard = cvar
            .wait_while(guard, |pending| !*pending)
            .unwrap();
        println!("notified 2 !");
    }));

    handles.push(thread::spawn(move || {
        let (lock, cvar) = &*pair1;
        let mut guard = lock
            .lock()
            .unwrap();
        while !*guard {
            guard = cvar
                .wait(guard)
                .unwrap();
        }
        println!("notified 3 !");
    }));

    for h in handles {
        let _ = h.join();
    }
}
