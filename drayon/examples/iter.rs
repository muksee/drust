use rayon::{
    current_thread_index,
    prelude::{
        IntoParallelIterator,
        ParallelIterator,
    },
    ThreadPoolBuilder,
};

fn main() {
    if let Err(e) = ThreadPoolBuilder::new()
        .num_threads(10)
        .build_global()
    {
        return;
    }

    (0..100)
        .into_par_iter()
        .for_each(|x| println!("{x}, {:?}", current_thread_index()));
}
