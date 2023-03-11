use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
// https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/9
use std::time::{SystemTime, UNIX_EPOCH};

static KEY: AtomicU64 = AtomicU64::new(0);

fn get_key() -> u64 {
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    nanos as u64
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                let key = get_key();
                println!("thread {i} gets {key}");
            });
        }
    });
}
