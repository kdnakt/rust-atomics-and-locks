use std::sync::atomic::{
    AtomicU64, // which is only 8 bytes
    Ordering::Relaxed,
};
use std::hint::black_box;
use std::thread;
use std::time::Instant;

#[repr(align(64))] // This struct must be 64-byte aligned.
struct Aligned(AtomicU64); // adds 56 bytes of padding

static A: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn main() {
    println!("Hello, world!");

    black_box(&A);
    thread::spawn(|| {
        loop {
            A[0].0.store(0, Relaxed);
            A[2].0.store(0, Relaxed);
        }
    });

    let start = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(A[1].0.load(Relaxed));
    }
    println!("{:?}", start.elapsed());
}
