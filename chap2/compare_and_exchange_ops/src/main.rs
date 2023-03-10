use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => {
                println!("expected={current}, got={v}");
                current = v;
            },
        }
    }
}

fn main() {
    println!("Hello, world!");

    let a = &AtomicU32::new(0);
    thread::scope(|s| {
        for i in 0..1000 {
            s.spawn(move || {
                increment(a);
                //println!("{i}: a={:?}", a);
            });
        }
    });
    println!("Done!");
}
