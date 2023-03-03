use std::sync::atomic::AtomicUsize;
use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    println!("Hello, world!");
    
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                thread::sleep(Duration::from_millis(i));
                num_done.store((i + 1).try_into().unwrap(), Relaxed);
                // wake up the main thread
                main_thread.unpark();
            }
        });
        // main thread shows status updates every second
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            // thread::sleep(Duration::from_secs(1));
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
