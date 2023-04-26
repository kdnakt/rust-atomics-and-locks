use std::sync::atomic::{
    AtomicBool,
};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::thread::Thread;
use std::marker::PhantomData;

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    // raw pointer does not implement Send,
    // not allowing to be sent between threads.
    _no_send: PhantomData<*const ()>,
}

fn main() {
    println!("Hello, world!");
}
