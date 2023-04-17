use std::sync::{
    Arc,
    atomic::AtomicBool,
};
use std::cell:UnsafeCell;
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
}

unsafe impl<T> Sync for Channel<T>
    where T: Send {}

fn main() {
    println!("Hello, world!");
}
