use std::sync::atomic::{
    AtomicBool,
    Ordering::{
        Acquire,
        Release,
    },
};
use std::thread;
use std::time::Duration;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    println!("Hello, world!");
    thread::spawn(|| {
        // Safety: nothing else is accessing DATA
        // because we haven't set the READY flag yet
        unsafe {
            DATA = 123;
        };
        READY.store(true, Release);
    });
    
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    // Safety: nothing is mutating DATA, because READY is set
    println!("{}", unsafe {
        DATA
    });
}
