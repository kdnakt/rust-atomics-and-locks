use std::thread;

/**
example output:
Hello from the main thread.
Hello from another thread!
This is my thread id: ThreadId(2)
Hello from another thread!
This is my thread id: ThreadId(3)

another output:
Hello from the main thread.
Hello from another thread!
This is my thread id: ThreadId(
*/
fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");
    
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
