use std::sync::atomic::AtomicUsize;
use std::cell::UnsafeCell;

struct ArcData<T> {
    /// number of `Arc`s
    data_ref_count: AtomicUsize,
    /// number of `Arc`s and `Weak`s combined
    alloc_ref_count: AtomicUsize,
    /// the data. `None` if there's only weak pointers left.
    data: UnsafeCell<Option<T>>,
}

fn main() {
    println!("Hello, world!");
}
