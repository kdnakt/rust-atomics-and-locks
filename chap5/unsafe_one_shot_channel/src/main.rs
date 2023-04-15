// unsafe version of Option<T>: requires its user to manually keep track of whether it is initialized
use std::mem::MaybeUninit;
use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Release,
        Acquire,
        Relaxed,
    },
};
use std::thread;

pub struct Channel<T> {
    // MaybeUninit will not automatically drop its contents
    message: UnsafeCell<MaybeUninit<T>>,
    // To indicate whether the channel has been taken in use
    in_use: AtomicBool,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

impl <T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    /// Panics when trying to send more than one message.
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("can't send more than one message!");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
        // this releases the message to the receiver
        // receiver will loads with Acquire ordering
    }

    pub fn is_ready(&self) -> bool {
        // now we have acquire-load of the flag in receive()
        self.ready.load(Relaxed)
    }

    /// Panics if no message is available yet,
    /// or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        // just checked and reset the ready flag
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            // below line will cause panic message!
            // but it's far better than undefined behavior
            // channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        let s = channel.receive();
        assert_eq!(s, "hello world!");
        println!("got: {:?}", s);
    });
}
