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
    /*
     * This is shorthand for
     * std::thread::Builder::new().spawn().unwrap(),
     * which returns std::io::Result.
     * The error may be caused by OOM, resource limits, etc.
     * thread::spawn() simply panics in that case.
     */
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

    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");
}

fn f() {
    println!("Hello from another thread!");
    
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
