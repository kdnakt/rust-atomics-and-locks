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

// type definition for our Mutex
pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32, // instead of an AtomicBool
                      // we can use it with the atomic wait and wake functions
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T>
    where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0), // unlocked state
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // set the state to 1: locked.
        while self.state.swap(1, Acquire) == 1 {
            // If it was already locked..
            //.. wait, unless the state is no longer 1.
            wait(&self.state, 1);
        }
        MutexGuard { mutex: self }
    }
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
