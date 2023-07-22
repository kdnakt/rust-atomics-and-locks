use std::sync::atomic::{
    AtomicU32,
    Ordering::{
        Acquire,
        Release,
    },
};
use std::cell::UnsafeCell;
use std::ops::{
    Deref,
    DerefMut,
};
use atomic_wait::{
    wait,
    wake_one,
};

pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked, no other threads waiting
    /// 2: locked, other threads waiting
    state: AtomicU32,
    value: UnsafeCell<T>,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Set the state back to 0: unlocked
        self.mutex.state.store(0, Release);
        // Wake up one of the waiting threads, if any.
        wake_one(&self.mutex.state);
    }
}

fn main() {
    println!("Hello, world!");
}
