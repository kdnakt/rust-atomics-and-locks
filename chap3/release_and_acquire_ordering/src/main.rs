use std::sync::atomic::{
    AtomicU64,
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
        Relaxed,
    },
};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    println!("Hello, world!");

    thread::spawn(|| {
        DATA.store(1, Relaxed);
        READY.store(true, Release); // Everything from before this store..
    });
    while !READY.load(Acquire) { // .. is visible after this loads true
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}
