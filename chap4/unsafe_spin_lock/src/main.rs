use std::cell::UnsafeCell;
use std::sync::atomic::{
    AtomicBool,
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

fn main() {
    println!("Hello, world!");
}
