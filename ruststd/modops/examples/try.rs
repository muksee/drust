#![feature(try_trait_v2)]

use std::ops::{
    ControlFlow,
    Try,
};

fn main() {}

fn simple_fold1<A, T>(
    iter: impl Iterator<Item = T>,
    mut accum: A,
    mut f: impl FnMut(A, T) -> A,
) -> A {
    for x in iter {
        accum = f(accum, x);
    }
    accum
}

fn simple_fold2<A, T, R: Try<Output = A>>(
    iter: impl Iterator<Item = T>,
    mut accum: A,
    mut f: impl FnMut(A, T) -> R,
) -> R {
    for x in iter {
        let cf = f(accum, x).branch();
        match cf {
            ControlFlow::Continue(a) => accum = a,
            ControlFlow::Break(r) => return R::from_residual(r),
        }
    }

    R::from_output(accum)
}
