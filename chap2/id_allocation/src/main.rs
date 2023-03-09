use std::sync::atomic::{
    AtomicU32,
    Ordering::Relaxed,
};
use std::thread;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

fn allocate_new_id() -> u32 {
    let id = NEXT_ID.fetch_add(1, Relaxed);
    // problematic
    // assert!(id < 1000, "too many IDs!");
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Relaxed);
        panic!("too many IDs!");
    }
    id
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                let id = allocate_new_id();
                println!("thread {i} gets id {id}");
            });
        }
    });
}

