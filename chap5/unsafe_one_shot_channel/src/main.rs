// unsafe version of Option<T>: requires its user to manually keep track of whether it is initialized
use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicBool;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

fn main() {
    println!("Hello, world!");
}
