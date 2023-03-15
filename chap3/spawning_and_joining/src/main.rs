use std::sync::atomic::{
    AtomicI32,
    Ordering::Relaxed,
};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn main() {
    println!("Hello, world!");

    X.store(1, Relaxed);
    // spawning creates a happens-before relationship
    // between what happened before the spawn() call
    // and the new thread.
    let t = thread::spawn(f);
    X.store(2, Relaxed);
    // Also join() creates happens-before relationship
    t.join().unwrap();
    X.store(3, Relaxed);
}

fn f() {
    let x = X.load(Relaxed);
    // cannot fail, because of spawn() and join()
    assert!(x == 1 || x == 2);
}