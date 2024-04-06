#![feature(try_trait_v2)]
#![feature(try_blocks)]
use std::ops::{
    ControlFlow,
    FromResidual,
    Try,
};

fn main() {
    // ------------------- 利用 enum 实现短路 -------------------
    // 短路逻辑:
    // 1.内部值大于100则短路,返回Wrong(x)
    // 2.内部值小于100则正常,返回Right(x)
    let t1: CircuitEnum<i32, i32> = try { CircuitEnum::<i32, i32>::new(1)? };
    let t2: CircuitEnum<i32, i32> = try { CircuitEnum::<i32, i32>::new(2)? };
    assert_eq!(t1, CircuitEnum::Right(1));
    assert_eq!(t2, CircuitEnum::Right(2));

    let t101: CircuitEnum<i32, i32> =
        try { CircuitEnum::<i32, i32>::new(101)? };
    let t102: CircuitEnum<i32, i32> =
        try { CircuitEnum::<i32, i32>::new(102)? };
    assert_eq!(t101, CircuitEnum::Wrong(101));
    assert_eq!(t102, CircuitEnum::Wrong(102));

    // ------------------- 利用struct实现短路 -------------------
    // 1.内部值大于100则短路,返回CircuitStruct(x)
    // 2.内部值小于100则正常,返回CircuitStruct(100)
    let t1: CircuitStruct<i32> = try { CircuitStruct::<i32>::new(1)? };
    let t2: CircuitStruct<i32> = try { CircuitStruct::<i32>::new(2)? };
    assert_eq!(t1, CircuitStruct(1));
    assert_eq!(t2, CircuitStruct(2));

    let t101: CircuitStruct<i32> = try { CircuitStruct::<i32>::new(101)? };
    let t102: CircuitStruct<i32> = try { CircuitStruct::<i32>::new(102)? };
    assert_eq!(t101, CircuitStruct(100));
    assert_eq!(t102, CircuitStruct(100));
}

#[derive(PartialEq, Eq, Debug)]
enum CircuitEnum<R, W> {
    Right(R),
    Wrong(W),
}

impl<R, W> CircuitEnum<R, W> {
    fn new(r: R) -> Self {
        Self::Right(r)
    }
}

impl Try for CircuitEnum<i32, i32> {
    type Output = u32;

    type Residual = u32;

    fn from_output(output: Self::Output) -> Self {
        Self::Right(output as i32)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            CircuitEnum::Right(r) => {
                if r < 100 {
                    ControlFlow::Continue(r as u32)
                } else {
                    ControlFlow::Break(r as u32)
                }
            }
            Self::Wrong(w) => {
                if w < 100 {
                    ControlFlow::Continue(w as u32)
                } else {
                    ControlFlow::Break(w as u32)
                }
            }
        }
    }
}

impl FromResidual for CircuitEnum<i32, i32> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Wrong(residual as i32)
    }
}

// ------------------- 利用struct实现短路 -------------------
// 短路逻辑: 内部值大于100则短路,并整流为100

#[derive(PartialEq, Eq, Debug)]
struct CircuitStruct<T>(T);

impl<T> CircuitStruct<T> {
    fn new(t: T) -> Self {
        CircuitStruct(t)
    }
}

impl Try for CircuitStruct<i32> {
    type Output = u32;

    type Residual = u32;

    fn from_output(output: Self::Output) -> Self {
        CircuitStruct((output * 2) as i32)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        if self.0 < 100 {
            ControlFlow::Continue(self.0 as u32)
        } else {
            ControlFlow::Break(100)
        }
    }
}

impl FromResidual for CircuitStruct<i32> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        CircuitStruct(residual as i32)
    }
}
