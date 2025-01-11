use std::{
    thread,
    time::Duration,
};

use rayon::{
    current_thread_index,
    in_place_scope,
    scope,
};

fn main() {
    {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .spawn_handler(|thread| {
                std::thread::spawn(|| thread.run());
                Ok(())
            })
            .start_handler(|n| {
                println!("start handle {n}");
            })
            .exit_handler(|n| {
                println!("exit handle {n}");
            })
            .build()
            .unwrap();

        pool.install(|| {
            println!(
                "install: Hello, from my custom thread: {:?}",
                current_thread_index()
            );
        });

        pool.broadcast(|cx| {
            println!("broadcast1: i am thread {}", cx.index());
        });

        pool.broadcast(|cx| {
            println!("broadcast2: i am thread {}", cx.index());
        });

        pool.join(
            || {
                println!(
                    "join: oper_a {:?}",
                    rayon::current_thread_index()
                )
            },
            || {
                println!(
                    "join: oper_a {:?}",
                    rayon::current_thread_index()
                )
            },
        );

        pool.spawn(|| {
            println!(
                "spawn: {:?}",
                rayon::current_thread_index()
            )
        });

        pool.scope(|s| {
            s.spawn(|s| {
                println!(
                    "scope: {:?}",
                    rayon::current_thread_index()
                )
            })
        });

        pool.in_place_scope(|s| {
            s.spawn(|s| {
                println!(
                    "in_place_scope: {:?}",
                    rayon::current_thread_index()
                )
            })
        });

        pool.install(|| {
            println!("install: {:?}", current_thread_index());
            in_place_scope(|s| {
                println!(
                    "in_place_scope: {:?}",
                    rayon::current_thread_index()
                )
            })
        });

        pool.install(|| {
            println!("install: {:?}", current_thread_index());
            scope(|s| {
                println!(
                    "in_place_scope: {:?}",
                    rayon::current_thread_index()
                )
            })
        })
    }
    thread::sleep(Duration::from_secs(2));
}
