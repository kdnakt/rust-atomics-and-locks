use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
                drop(guard); // Drop the guard before sleeping, whole program takes about 1 sec.
                thread::sleep(Duration::from_secs(1));
            });
        }
    });
    // into_inner(): safely remove the protection, take ownership of the mutex
    let unwrapped = n.into_inner().unwrap();
    println!("{}", unwrapped);
    assert_eq!(unwrapped, 1000);
}
