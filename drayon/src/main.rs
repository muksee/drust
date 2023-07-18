use rayon::prelude::{
    IntoParallelIterator,
    ParallelIterator,
};

fn main() {
    (0..100)
        .into_par_iter()
        .for_each(|x| println!("{x:?}"));
}
