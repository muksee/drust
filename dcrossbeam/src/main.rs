use std::sync::Arc;

fn main() {
    let map: Arc<crossbeam_skiplist::SkipMap<u32, u32>> =
        Arc::new(crossbeam_skiplist::SkipMap::new());
}
