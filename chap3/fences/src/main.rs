use std::sync::atomic::{
    fence,
    AtomicBool,
    Ordering::{
        Relaxed,
        Acquire,
        Release,
    },
};
use std::time::Duration;
use std::thread;

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn some_calculation(i: usize) -> u64 {
    i as u64 * 2
}

fn main() {
    println!("Hello, world!");
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Release);
        });
    }
    // thread::sleep(Duration::from_millis(100));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }
}
