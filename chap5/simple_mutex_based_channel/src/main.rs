use std::sync::{
    Mutex,
    Condvar,
};
use std::collections::VecDeque;
use std::thread;

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl <T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }
    
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let channel: Channel<i32> = Channel::new();
    thread::scope(|s| {
        s.spawn(|| {
            println!("Sending 1");
            channel.send(1);
            println!("Sent 1, Sending 2");
            channel.send(2);
            println!("Sent 2");
        });
        s.spawn(|| {
            println!("Waiting");
            println!("Receiving {}", channel.receive());
            println!("Receiving {}", channel.receive());
            println!("end of thread");
        });
    });
    println!("Done.");
}
