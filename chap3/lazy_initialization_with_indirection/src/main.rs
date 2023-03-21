use std::sync::atomic::{
    AtomicPtr,
    Ordering::{
        Acquire,
        Release,
    },
};
use std::thread;

#[derive(Debug)]
struct Data {}

static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

fn get_data(i: i32) -> &'static Data {
    println!("thread {i}: start");
    let mut p = PTR.load(Acquire);
    if p.is_null() {
        println!("thread {i}: p is null");
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(), p, Release, Acquire
        ) {
            println!("thread {i}: compare_exchange error");
            // Safety: p comes from Box::into_raw right above,
            // and wasn't shared with any other thread
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value
    unsafe { &*p }
}

fn generate_data() -> Data {
    Data {}
}

fn main() {
    println!("Hello, world!");

    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                let data = get_data(i);
                println!("thread {i}: {:?}", data);
            });
        }
    });

    println!("Done!");
}
