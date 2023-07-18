use std::ops::{
    Index,
    IndexMut,
};

#[allow(dead_code)]
#[non_exhaustive]
enum Neucleotide {
    A,
    C,
    G,
    T,
}

struct NeucleotideCount {
    a: usize,
    c: usize,
    g: usize,
    t: usize,
}

impl Index<Neucleotide> for NeucleotideCount {
    type Output = usize;

    fn index(&self, index: Neucleotide) -> &Self::Output {
        match index {
            Neucleotide::A => &self.a,
            Neucleotide::C => &self.c,
            Neucleotide::G => &self.g,
            Neucleotide::T => &self.t,
        }
    }
}

impl IndexMut<Neucleotide> for NeucleotideCount {
    fn index_mut(&mut self, index: Neucleotide) -> &mut Self::Output {
        match index {
            Neucleotide::A => &mut self.a,
            Neucleotide::C => &mut self.c,
            Neucleotide::G => &mut self.g,
            Neucleotide::T => &mut self.t,
        }
    }
}

fn main() {
    let mut nc = NeucleotideCount {
        a: 100,
        c: 200,
        g: 300,
        t: 400,
    };
    // * nc.index(Neucleotide::A)
    println!("a: {}", nc[Neucleotide::A]);

    // * nc.index_mut(Neucleotide::A) = 1000
    nc[Neucleotide::A] = 1000;
    println!("a: {}", nc[Neucleotide::A]);
}
