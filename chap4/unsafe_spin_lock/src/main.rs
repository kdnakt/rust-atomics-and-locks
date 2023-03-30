use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    // UnsafeCel doesn't implement Sync: not shareable btw threads
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T>
    // only allow one thread at a time to access the T
    // so no need T to implement Sync
    where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
