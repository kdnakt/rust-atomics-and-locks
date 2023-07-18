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

fn main() {
    println!("Hello, world!");
}
