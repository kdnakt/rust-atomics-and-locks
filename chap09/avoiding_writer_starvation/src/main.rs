use std::sync::atomic::AtomicU32;
use std::cell::UnsafeCell;

pub struct RwLock<T> {
    /// The number of read locks times two,
    /// plus one if there's a writer waiting.
    /// u32::MAX if write locked.
    /// 
    /// This means that readers may acquire the lock
    /// when the state is even, but need to block when odd.
    state: AtomicU32,
    /// Incremented to wake up writers.
    writer_wake_counter: AtomicU32,
    value: UnsafeCell<T>,
}

fn main() {
    println!("Hello, world!");
}
