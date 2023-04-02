use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
    },
};
use std::ops::{
    Deref,
    DerefMut,
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
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
    // Safety: The &mut T from lock() must be gone!
    // And no cheating by keeping reference to fields of that T around!
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

// We need to tie the unlocking operation to the end of the &mut T
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// to make Guard<T> behave like an (exclusive) reference
// that is, transparently give access to T
impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: the very existence of this Guard
        // guarantees we've exclusively locked the lock
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: the very existence of this Guard
        // guarantees we've exclusively locked the lock
        unsafe { &mut *self.lock.value.get() }
    }
}

fn main() {
    println!("Hello, world!");
}
