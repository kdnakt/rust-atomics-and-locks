use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Condvar;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    // takes care of delivering the notifications to threads
    // IMPORTANT: Condvar is only used with a single Mutex, two different mutexes might cause panic
    let not_empty = Condvar::new();

    thread::scope(|s| {
        // consuming thread
        // let t = s.spawn(|| loop {
        s.spawn(|| {
            loop {
                let mut q = queue.lock().unwrap();
                let item = loop {
                    if let Some(item) = q.pop_front() {
                        break item;
                    } else {
                        // wait() takes MutexGuard: first unlocks it and wait, then relocks with a new MutexGuard
                        q = not_empty.wait(q).unwrap();
                        // Also we have Condvar::wait_timeout(Duration)
                    }
                };
                drop(q);
                dbg!(item);
                // if let Some(item) = item {
                //     dbg!(item);
                // } else {
                //     // sleep, stop consuming any CPU cycles
                //     // returns when it gets unparked
                //     thread::park();
                //     // IMPORTANT: park() does not guarantee
                //     // that it will only return because of a matching unpark()
                //      We also have thread::park_timeout(Duration)
                // }
            }
        });

        // producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            queue.lock().unwrap().push_back(i);
            // No need to know which thread to wake up
            not_empty.notify_one();
            // // unpark the parked thread, waking it up
            // t.thread().unpark();
            // // if unpark() called before park(), park() will handle it without sleep.
            // // However, unpark requests don't stack up: first park() clears the request
            // // So we only park the thread if the queue is empty
            thread::sleep(Duration::from_secs(1));
        }
    });
}
