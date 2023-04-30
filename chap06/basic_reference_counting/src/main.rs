use std::sync::atomic::AtomicUsize;
use std::ptr::NonNull;

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

// Not a Box, which is exclusive ownership.
// We use pointer for shared ownership.
pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

fn main() {
    println!("Hello, world!");
}
