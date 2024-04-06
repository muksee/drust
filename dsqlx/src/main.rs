// #![no_std]
// #![feature(generic_const_exprs)]

// pub trait Bits {
//     const BITS: usize;

//     type Underlying;

//     fn from_arbint(from: arbitrary_int::UInt<Self::Underlying, { Self::BITS }>) -> Self;
// }

// impl Bits for bool {
//     const BITS: usize = 1;

//     type Underlying = u8;

//     fn from_arbint(from: arbitrary_int::UInt<Self::Underlying, { Self::BITS }>) -> Self {
//         from.value() == 1
//     }
// }

// impl<T, const BITS: usize> Bits for arbitrary_int::UInt<T, BITS> {
//     const BITS: usize = BITS;

//     type Underlying = arbitrary_int::UInt<T, BITS>;

//     fn from_arbint(from: arbitrary_int::UInt<Self::Underlying, { Self::BITS }>) -> Self {
//         from
//     }
// }

// pub trait CalcUint {
//     type Uint;
// }
// impl CalcUint for [(); 1] {
//     type Uint = u8;
// }
// impl CalcUint for [(); 9] {
//     type Uint = u16;
// }

// impl<T, const N: usize> Bits for [T; N]
// where
//     T: Bits + Default + Copy,
//     [(); T::BITS * N]: CalcUint,
// {
//     const BITS: usize = T::BITS * N;

//     // type Underlying = <[(); T::BITS * N] as CalcUint>::Uint;
//     type Underlying = u8;

//     fn from_arbint(from: arbitrary_int::UInt<Self::Underlying, { Self::BITS }>) -> Self {
//         todo!()
//     }
// }

fn main() {
    let s = "hello";
    let r = life(s);
}


fn life<'a, 'b>(input: &'a str) -> &'b str {
    "world"
}


#[cfg(test)]
mod test {
    #[test]
    fn p() {
        panic!("hello world");
    }

    #[test]
    fn pp() {
        println!("hello world");
    }
}

