use std::sync::{
    Arc,
    atomic::{
        AtomicBool,
        Ordering::{
            Acquire,
            Release,
            Relaxed,
        },
    },
};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    /// This never panics
    pub fn send(self, message: T) {
        unsafe {(*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

// no longer `pub`
struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    // in_use is no longer needed since type system guarantees that
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop()
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (Sender { channel: a.clone() }, Receiver { channel: a })
}

fn main() {
    println!("Hello, world!");
}
