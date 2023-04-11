// unsafe version of Option<T>: requires its user to manually keep track of whether it is initialized
use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Release,
        Acquire,
    },
};

pub struct Channel<T> {
    // MaybeUninit will not automatically drop its contents
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

    pub fn is_ready(&self) -> bool {
        self.ready.load(Acquire)
    }

    /// Panics if no message is available yet.
    ///
    /// Tip: Use `is_ready` to check first.
    ///
    /// Safety: Only call this once
    pub unsafe fn receive(&self) -> T {
        if !self.ready.load(Acquire) {
            panic!("no message available!");
        }
        (*self.message.get()).assume_init_read()
    }
}

fn main() {
    println!("Hello, world!");
}
