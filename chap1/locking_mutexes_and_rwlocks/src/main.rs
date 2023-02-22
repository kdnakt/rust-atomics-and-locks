use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock()
                    // unwrap() calls relate to lock poisoning
                    // if a thread panics, mutex gets marked as poisoned
                    // and lock() will return Err
                    .unwrap();
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

    let list = Mutex::new(vec![1, 2, 3]);
    // lock, push, unlock in a single statement
    list.lock().unwrap().push(1);
    println!("{:?}", list);

    if let Some(item) = list.lock().unwrap().pop() {
        println!("{item}");
    }; // MutexGuard is unlocked here, at the end of the if let statement
}
