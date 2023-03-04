use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
use std::sync::Once;

fn calculate_x() -> u64 {
    thread::sleep(Duration::from_secs(3));
    println!("calculation finished!");
    3
}

static mut X: u64 = 0;
static INIT: Once = Once::new();

fn get_x() -> u64 {
    unsafe {
        INIT.call_once(|| {
            X = calculate_x();
        });
        X
    }
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
