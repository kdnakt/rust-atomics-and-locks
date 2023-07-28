use std::sync::atomic::{
    AtomicU32,
    Ordering::Relaxed,
};
use atomic_wait::{
    wake_one,
    wake_all,
};

pub struct Condvar {
    counter: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self {
        Self { counter: AtomicU32::new(0) }
    }

    pub fn notify_one(&self) {
        self.counter.fetch_add(1, Relaxed);
        wake_one(&self.counter);
    }

    pub fn notify_all(&self) {
        self.counter.fetch_add(1, Relaxed);
        wake_all(&self.counter);
    }
}

fn main() {
    println!("Hello, world!");
}
