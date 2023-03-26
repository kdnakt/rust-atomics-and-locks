use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
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
        while self.locked.swap(true, Acquire) {
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
