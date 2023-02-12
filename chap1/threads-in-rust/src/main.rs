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

    /*
     * The println macro uses std::io::Stdout::lock()
     * to make sure its output does not get interrupted.
     */
    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();

    // pass a closure
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}

fn f() {
    println!("Hello from another thread!");
    
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
