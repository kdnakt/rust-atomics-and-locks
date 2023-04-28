use std::sync::atomic::AtomicUsize;

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

fn main() {
    println!("Hello, world!");
}
