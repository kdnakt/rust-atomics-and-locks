use std::sync::Mutex;

fn main() {
    println!("Hello, world!");
    
    let m = Mutex::new(1);
    
    let guard = m.lock(); // lock it,
    std::mem::forget(guard); // but don't unlock it.
} // m will be dropped (destroyed) at the end of the scope
// but it is still locked
