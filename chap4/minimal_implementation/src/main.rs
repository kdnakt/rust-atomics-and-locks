use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
        Relaxed,
    },
};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }

    pub fn lock(&self) {
        // while self.locked.swap(true, Acquire) {
        // or we can use compare-and-exchange
        while self.locked.compare_exchange_weak(
            false, true, Acquire, Relaxed
        ).is_err() {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");

    let spin_lock = SpinLock::new();
    println!("locked: {:?}", spin_lock.locked);
    spin_lock.lock();
    println!("locked: {:?}", spin_lock.locked);
    spin_lock.unlock();
    println!("locked: {:?}", spin_lock.locked);
}
