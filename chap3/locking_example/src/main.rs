use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Relaxed,
        Release,
    },
};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
        // Safety: we hold the exclusive lock, so nothing else is accessing DATA
        unsafe { DATA.push('!') };
        LOCKED.store(false, Release);
    }
}

fn main() {
    println!("Hello, world!");
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(f);
        }
    });
    println!("{}", unsafe { DATA.to_string() });
}
