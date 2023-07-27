use std::sync::atomic::AtomicU32;

pub struct Condvar {
    counter: AtomicU32,
}

fn main() {
    println!("Hello, world!");
}
