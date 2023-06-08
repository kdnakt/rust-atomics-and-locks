use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Instant;
use std::hint::black_box;
use std::thread;

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    println!("Hello, world!");
    black_box(&A);

    thread::spawn(|| {
        loop {
            // This has no effect
            // black_box(A.load(Relaxed));

            A.store(0, Relaxed);
        }
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
