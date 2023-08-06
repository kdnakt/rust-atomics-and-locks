use std::sync::atomic::AtomicU32;
use std::cell::UnsafeCell;

pub struct RwLock<T> {
    /// The number of readers, or u32::MAX if write-locked.
    state: AtomicU32,
    /// Incremented to wake up writers.
    writer_wake_counter: AtomicU32, // New!
    value: UnsafeCell<T>,
}

fn main() {
    println!("Hello, world!");
}
