use std::sync::atomic::AtomicU32;

pub struct Condvar {
    counter: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self {
        Self { counter: AtomicU32::new(0) }
    }
}

fn main() {
    println!("Hello, world!");
}
