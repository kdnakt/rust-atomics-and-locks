// unsafe version of Option<T>: requires its user to manually keep track of whether it is initialized
use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
    Ordering::Release,
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

impl <T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    // Safety: only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
        // this releases the message to the receiver
        // receiver will loads with Acquire ordering
    }
}

fn main() {
    println!("Hello, world!");
}
