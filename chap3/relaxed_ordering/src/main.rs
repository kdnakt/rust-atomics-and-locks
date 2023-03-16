use std::sync::atomic::{
    AtomicI32,
    Ordering::Relaxed,
};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{a} {b} {c} {d}");
    // threads can't observe the result inconsistent with total modification order, defined in a()
    // 0 0 5 15 is possible but 0 5 0 15 is impossible
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        s.spawn(|| {
            a();
        });
        s.spawn(|| {
            b();
        });
    });
}
