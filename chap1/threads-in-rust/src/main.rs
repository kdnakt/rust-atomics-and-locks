use std::thread;

/**
example output:
Hello from the main thread.
Hello from another thread!
This is my thread id: ThreadId(2)
Hello from another thread!
This is my thread id: ThreadId(3)

another output without JoinHandle:
Hello from the main thread.
Hello from another thread!
This is my thread id: ThreadId(
*/
fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread!");
    
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
