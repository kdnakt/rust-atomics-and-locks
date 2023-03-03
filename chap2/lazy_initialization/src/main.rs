use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn calculate_x() -> u64 {
    thread::sleep(Duration::from_secs(3));
    println!("calculation finished!");
    3
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        s.spawn(|| {
            println!("x={}", get_x());
        });
        s.spawn(|| {
            println!("x={}", get_x());
        });
        s.spawn(|| {
            println!("x={}", get_x());
        });
    });

    println!("Done!");
}
