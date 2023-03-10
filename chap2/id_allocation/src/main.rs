use std::sync::atomic::{
    AtomicU32,
    Ordering::Relaxed,
};
use std::thread;

static NEXT_ID: AtomicU32 = AtomicU32::new(0);

fn allocate_new_id() -> u32 {
    // let id = NEXT_ID.fetch_add(1, Relaxed);
    // // problematic
    // // assert!(id < 1000, "too many IDs!");
    // if id >= 1000 {
    //     NEXT_ID.fetch_sub(1, Relaxed);
    //     panic!("too many IDs!");
    // }
    // id

    // use compare and exchange
    // let mut id = NEXT_ID.load(Relaxed);
    // loop {
    //     // if we want, u32::MAX, without having to worry about edge cases
    //     assert!(id < 1000, "too many IDs!");
    //     match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
    //         Ok(_) => return id,
    //         Err(v) => {
    //             println!("expected={id}, got={v}");
    //             id = v;
    //         },
    //     }
    // }

    // one-liner with fetch_update() method
    NEXT_ID.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1)).expect("too many IDs!")
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        for i in 0..100 {
            s.spawn(move || {
                let id = allocate_new_id();
                println!("thread {i} gets id {id}");
            });
        }
    });

    println!("Done!");
}

