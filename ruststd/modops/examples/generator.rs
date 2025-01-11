#![feature(generators, generator_trait)]

use std::{
    ops::{
        Generator,
        GeneratorState,
    },
    pin::Pin,
};

fn main() {
    let mut generator = || {
        yield 1;
        yield 2;
        "foo"
    };

    loop {
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(x) => {
                println!("yield: {x}");
            }
            GeneratorState::Complete(x) => {
                println!("complete: {x}");
            }
        }
    }
}
