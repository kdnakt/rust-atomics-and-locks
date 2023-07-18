use std::sync::atomic::AtomicU32;
use std::cell::UnsafeCell;

// type definition for our Mutex
pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32, // instead of an AtomicBool
                      // we can use it with the atomic wait and wake functions
    value: UnsafeCell<T>,
}

unsafe impl<T> for Mutex<T>
    where T: Send {}

fn main() {
    println!("Hello, world!");
}
