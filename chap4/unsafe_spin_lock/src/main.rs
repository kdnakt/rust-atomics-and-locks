use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
    },
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
    // we can make the lifetimes explicit
    // like: pub fn lock<'a>(&'a self) -> &'a mut T { ... }
    // the returned reference is valid as long as the lock exists.
    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        unsafe { &mut *self.value.get() }
    }
    // Safety: The &mut T from lock() must be gone!
    // And no cheating by keeping reference to fields of that T around!
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");
}
