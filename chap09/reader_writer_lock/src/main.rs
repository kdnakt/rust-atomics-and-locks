use std::sync::atomic::AtomicU32;
use std::cell::UnsafeCell;

pub struct RwLock<T> {
    /// The number of readers, or u32::MAX if write-locked.
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RwLock<T>
    where T: Send + Sync {}

fn main() {
    println!("Hello, world!");
}
