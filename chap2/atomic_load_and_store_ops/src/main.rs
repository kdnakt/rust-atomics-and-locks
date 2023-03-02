use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

/**
 * impl AtomicI32 {
 *   pub fn load(&self, ordering: Ordering) -> i32;
 *   pub fn store(&self, value: i32, ordering: Ordering)
 * }
 */
fn main() {
    println!("Hello, world!");

    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread = std::thread::spawn(|| {
        while !STOP.load(Relaxed) {
            println!("still working ...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    // Inform the background thread
    STOP.store(true, Relaxed);
    // Wait until the background thread finishes
    background_thread.join().unwrap();
}
