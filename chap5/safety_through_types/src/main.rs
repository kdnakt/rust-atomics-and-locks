use std::sync::{
    Arc,
    atomic::AtomicBool,
};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

// no longer `pub`
struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    // in_use is no longer needed since type system guarantees that
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

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
